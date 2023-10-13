use super::*;
use std::{fmt, str::FromStr};

/// An owned JSON value.
#[derive(Clone, PartialEq, Eq)]
pub struct Value {
    pub(crate) buffer: Box<[u8]>,
    pub(crate) id: Id,
}

impl Value {
    /// Returns the value as a reference.
    pub fn as_ref(&self) -> ValueRef<'_> {
        ValueRef::from(&self.buffer, self.id)
    }

    /// If the value is `null`, returns `()`. Returns `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        self.as_ref().as_null()
    }

    /// If the value is a boolean, returns the associated bool. Returns `None` otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        self.as_ref().as_bool()
    }

    /// If the value is an integer, returns the associated i64. Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        self.as_ref().as_i64()
    }

    /// If the value is a float, returns the associated f64. Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        self.as_ref().as_f64()
    }

    /// If the value is a string, returns the associated str. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        self.as_ref().as_str()
    }

    /// If the value is an array, returns the associated array. Returns `None` otherwise.
    pub fn as_array(&self) -> Option<ArrayRef<'_>> {
        self.as_ref().as_array()
    }

    /// If the value is an object, returns the associated map. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<ObjectRef<'_>> {
        self.as_ref().as_object()
    }

    /// Dumps the internal buffer as a string.
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

/// Display a JSON value as a string.
impl fmt::Display for Value {
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
        builder.finish(id)
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
        use ::serde::de::DeserializeSeed;

        let mut builder = Builder::new();
        let mut deserializer = serde_json::Deserializer::from_str(s);
        let id = builder.deserialize(&mut deserializer)?;
        Ok(builder.finish(id))
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_null();
        builder.finish(id)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_bool(b);
        builder.finish(id)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_i64(v);
        builder.finish(id)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_f64(v);
        builder.finish(id)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let mut builder = Builder::default();
        let id = builder.add_string(s);
        builder.finish(id)
    }
}

impl From<ValueRef<'_>> for Value {
    fn from(v: ValueRef<'_>) -> Self {
        let mut builder = Builder::with_capacity(v.capacity());
        let id = builder.add_value_ref(v);
        builder.finish(id)
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
#0: "null"
#9: "bool"
#18: "string"
#29: "hello"
#39: "integer"
#51: 43
#60: "float"
#70: 178.5
#79: "array"
#89: "hello"
#99: "world"
#109: [#89, #99]
#122: {#0:null, #9:true, #18:#29, #39:#51, #60:#70, #79:#109}
"#
            .trim()
        );
    }
}
