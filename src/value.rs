use super::*;
use std::{fmt, str::FromStr};

pub struct Value {
    buffer: Box<[u8]>,
    id: Id,
}

impl Value {
    fn as_ref(&self) -> ValueRef<'_> {
        ValueRef {
            buffer: &self.buffer,
            id: self.id,
        }
    }

    pub fn dump(&self) -> String {
        dump(&self.buffer)
    }

    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        Self::from(&value)
    }
}

impl From<&serde_json::Value> for Value {
    fn from(value: &serde_json::Value) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_serde_value(&value);
        Value {
            buffer: builder.into_buffer().into(),
            id,
        }
    }
}

impl Builder {
    /// Adds a serde `Value` recursively to the builder and returns its ID.
    fn add_serde_value(&mut self, value: &serde_json::Value) -> Id {
        match value {
            serde_json::Value::Null => self.add_null(),
            serde_json::Value::Bool(b) => self.add_bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    self.add_i64(i)
                } else if let Some(f) = n.as_f64() {
                    self.add_f64(f)
                } else {
                    panic!("invalid number");
                }
            }
            serde_json::Value::String(s) => self.add_string(s),
            serde_json::Value::Array(a) => {
                let ids = a
                    .iter()
                    .map(|v| self.add_serde_value(v))
                    .collect::<Vec<Id>>();
                self.add_array(&ids)
            }
            serde_json::Value::Object(o) => {
                let kvs = o
                    .iter()
                    .map(|(k, v)| (k.as_str(), self.add_serde_value(v)))
                    .collect::<Vec<(&str, Id)>>();
                self.add_object(kvs.into_iter())
            }
        }
    }
}

impl FromStr for Value {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: directly parse into the builder
        let serde_value = serde_json::Value::from_str(s)?;
        Ok(serde_value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_serde() {
        let serde_value: serde_json::Value = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#
        .parse()
        .unwrap();
        let value = Value::from(&serde_value);
        value.dump();
    }

    #[test]
    fn dump() {
        let value: Value = r#"
        {
            "null": null,
            "bool": true,
            "string": "hello",
            "integer": 43,
            "float": 178.5,
            "array": ["hello", "world"]
        }"#
        .parse()
        .unwrap();
        // println!("{}", value.dump());
        assert_eq!(
            value.dump().trim(),
            r#"
#0: "hello"
#10: "world"
#20: [#0, #10]
#33: 178.5
#42: 43
#51: "array"
#61: "bool"
#70: "float"
#80: "integer"
#92: "null"
#101: "string"
#112: {#51:#20, #61:true, #70:#33, #80:#42, #92:null, #101:#0}
"#
            .trim()
        );
    }
}
