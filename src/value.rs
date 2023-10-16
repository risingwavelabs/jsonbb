use super::*;
use std::{fmt, str::FromStr};

/// An owned JSON value.
#[derive(Clone)]
pub struct Value {
    pub(crate) buffer: Box<[u8]>,
}

impl Value {
    /// Returns a reference to the value.
    pub fn as_ref(&self) -> ValueRef<'_> {
        unsafe {
            let base = self.buffer.as_ptr().add(self.buffer.len() - 4);
            let entry = (base as *const Entry).read();
            ValueRef::from_raw(base, entry)
        }
    }

    /// If the value is `null`, returns `()`. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from(());
    /// assert_eq!(value.as_null(), Some(()));
    /// ```
    pub fn as_null(&self) -> Option<()> {
        self.as_ref().as_null()
    }

    /// If the value is a boolean, returns the associated bool. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from(true);
    /// assert_eq!(value.as_bool(), Some(true));
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        self.as_ref().as_bool()
    }

    /// If the value is an integer, returns the associated i64. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from(1i64);
    /// assert_eq!(value.as_i64(), Some(1));
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        self.as_ref().as_i64()
    }

    /// If the value is an integer, returns the associated u64. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from(1i64);
    /// assert_eq!(value.as_u64(), Some(1));
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        self.as_ref().as_u64()
    }

    /// If the value is a float, returns the associated f64. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from(3.14_f64);
    /// assert_eq!(value.as_f64(), Some(3.14));
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        self.as_ref().as_f64()
    }

    /// If the value is a string, returns the associated str. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = flat_json::Value::from("json");
    /// assert_eq!(value.as_str(), Some("json"));
    /// ```
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

    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Index into a JSON array or object.
    /// A string index can be used to access a value in an object,
    /// and a usize index can be used to access an element of an array.
    ///
    /// # Example
    ///
    /// ```
    /// let object: flat_json::Value = r#"{"a": 1, "b": 2}"#.parse().unwrap();
    /// assert_eq!(object.get("a").unwrap().to_string(), "1");
    /// assert!(object.get("c").is_none());
    /// assert!(object.get(0).is_none());
    ///
    /// let array: flat_json::Value = r#"["a", "b"]"#.parse().unwrap();
    /// assert_eq!(array.get(0).unwrap().to_string(), "\"a\"");
    /// assert!(array.get(2).is_none());
    /// assert!(array.get("a").is_none());
    /// ```
    pub fn get(&self, index: impl Index) -> Option<ValueRef<'_>> {
        index.index_into(&self.as_ref())
    }

    fn from_builder(capacity: usize, f: impl FnOnce(&mut Builder)) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        let mut builder = Builder::new(&mut buffer);
        f(&mut builder);
        builder.finish();
        Self {
            buffer: buffer.into_boxed_slice(),
        }
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
        Self::from_builder(0, |b| b.add_serde_value(value))
    }
}

impl Builder<'_> {
    /// Adds a serde `Value` recursively to the builder and returns its ptr.
    fn add_serde_value(&mut self, value: &serde_json::Value) {
        match value {
            serde_json::Value::Null => self.add_null(),
            serde_json::Value::Bool(b) => self.add_bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_u64() {
                    self.add_u64(i)
                } else if let Some(i) = n.as_i64() {
                    self.add_i64(i)
                } else if let Some(f) = n.as_f64() {
                    self.add_f64(f)
                } else {
                    panic!("invalid number");
                }
            }
            serde_json::Value::String(s) => self.add_string(s),
            serde_json::Value::Array(a) => {
                self.begin_array();
                for v in a.iter() {
                    self.add_serde_value(v);
                }
                self.finish_array();
            }
            serde_json::Value::Object(o) => {
                self.begin_object();
                for (k, v) in o.iter() {
                    self.add_string(k);
                    self.add_serde_value(v);
                }
                self.finish_object()
            }
        }
    }
}

impl FromStr for Value {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ::serde::de::DeserializeSeed;

        let mut buffer = Vec::with_capacity(s.len());
        let mut builder = Builder::new(&mut buffer);
        let mut deserializer = serde_json::Deserializer::from_str(s);
        builder.deserialize(&mut deserializer)?;
        builder.finish();
        Ok(Value {
            buffer: buffer.into_boxed_slice(),
        })
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::from_builder(4, |b| b.add_null())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::from_builder(4, |b| b.add_bool(v))
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::from_builder(1 + 8 + 4, |b| b.add_u64(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::from_builder(1 + 8 + 4, |b| b.add_i64(v))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::from_builder(1 + 8 + 4, |b| b.add_f64(v))
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::from_builder(s.len() + 8, |b| b.add_string(s))
    }
}

impl From<ValueRef<'_>> for Value {
    fn from(v: ValueRef<'_>) -> Self {
        Self::from_builder(v.capacity() + 4, |b| b.add_value_ref(v))
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
        let _value = Value::from(&serde_value);
    }
}
