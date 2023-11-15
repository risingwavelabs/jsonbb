// Copyright 2023 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use bytes::BufMut;
use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

/// An owned JSON value.
#[derive(Clone)]
pub struct Value {
    pub(crate) buffer: Box<[u8]>,
}

impl Value {
    /// Returns a `null` value.
    pub fn null() -> Self {
        Self::from(())
    }

    /// Creates a new JSON array from an iterator of values.
    pub fn array<'a>(iter: impl IntoIterator<Item = ValueRef<'a>>) -> Self {
        Self::from_builder(0, |b| {
            b.begin_array();
            for v in iter {
                b.add_value(v);
            }
            b.end_array();
        })
    }

    /// Creates a new JSON object from an iterator of key-value pairs.
    pub fn object<'a>(iter: impl IntoIterator<Item = (&'a str, ValueRef<'a>)>) -> Self {
        Self::from_builder(0, |b| {
            b.begin_object();
            for (k, v) in iter {
                b.add_string(k);
                b.add_value(v);
            }
            b.end_object();
        })
    }

    /// Deserialize an instance of `Value` from bytes of JSON text.
    pub fn from_text(json: &[u8]) -> serde_json::Result<Self> {
        use ::serde::de::DeserializeSeed;

        let mut builder = Builder::with_capacity(json.len());
        let mut deserializer = serde_json::Deserializer::from_slice(json);
        builder.deserialize(&mut deserializer)?;
        deserializer.end()?;
        Ok(builder.finish())
    }

    /// Deserialize an instance of `Value` from bytes of JSON text.
    #[cfg(feature = "simd-json")]
    pub fn from_text_mut(json: &mut [u8]) -> simd_json::Result<Self> {
        use ::serde::de::DeserializeSeed;

        let mut builder = Builder::with_capacity(json.len());
        let mut deserializer = simd_json::Deserializer::from_slice(json)?;
        builder.deserialize(&mut deserializer)?;
        Ok(builder.finish())
    }

    /// Creates a JSON `Value` from bytes of jsonbb encoding.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            buffer: bytes.into(),
        }
    }

    /// Returns a reference to the value.
    pub fn as_ref(&self) -> ValueRef<'_> {
        ValueRef::from_bytes(&self.buffer)
    }

    /// Returns the value as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// If the value is `null`, returns `()`. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value = jsonbb::Value::from(());
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
    /// let value = jsonbb::Value::from(true);
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
    /// let value = jsonbb::Value::from(1i64);
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
    /// let value = jsonbb::Value::from(1i64);
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
    /// let value = jsonbb::Value::from(3.14_f64);
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
    /// let value = jsonbb::Value::from("json");
    /// assert_eq!(value.as_str(), Some("json"));
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        self.as_ref().as_str()
    }

    /// If the value is an array, returns the associated array. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value: jsonbb::Value = "[]".parse().unwrap();
    /// assert_eq!(value.as_array().unwrap().len(), 0);
    /// ```
    pub fn as_array(&self) -> Option<ArrayRef<'_>> {
        self.as_ref().as_array()
    }

    /// If the value is an object, returns the associated map. Returns `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let value: jsonbb::Value = "{}".parse().unwrap();
    /// assert_eq!(value.as_object().unwrap().len(), 0);
    /// ```
    pub fn as_object(&self) -> Option<ObjectRef<'_>> {
        self.as_ref().as_object()
    }

    /// Returns true if the value is a null. Returns false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(()).is_null());
    ///
    /// // The boolean `false` is not null.
    /// assert!(!jsonbb::Value::from(false).is_null());
    /// ```
    pub fn is_null(self) -> bool {
        self.as_ref().is_null()
    }

    /// Returns true if the value is a boolean. Returns false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(false).is_boolean());
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert!(!jsonbb::Value::from("false").is_boolean());
    /// ```
    pub fn is_boolean(self) -> bool {
        self.as_ref().is_boolean()
    }

    /// Returns true if the value is a number. Returns false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(1).is_number());
    ///
    /// // The string `"1"` is a string, not a number.
    /// assert!(!jsonbb::Value::from("1").is_number());
    /// ```
    pub fn is_number(self) -> bool {
        self.as_ref().is_number()
    }

    /// Returns true if the value is an integer between zero and `u64::MAX`.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(1i64).is_u64());
    ///
    /// // Negative integer.
    /// assert!(!jsonbb::Value::from(-1i64).is_u64());
    /// ```
    pub fn is_u64(self) -> bool {
        self.as_ref().is_u64()
    }

    /// Returns true if the value is an integer between `i64::MIN` and `i64::MAX`.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(1u64).is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!jsonbb::Value::from(u64::MAX).is_i64());
    /// ```
    pub fn is_i64(self) -> bool {
        self.as_ref().is_i64()
    }

    /// Returns true if the value is a number that can be represented by f64.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from(0f64).is_f64());
    ///
    /// // Integer
    /// assert!(!jsonbb::Value::from(1i64).is_f64());
    /// ```
    pub fn is_f64(self) -> bool {
        self.as_ref().is_f64()
    }

    /// Returns true if the value is a string. Returns false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(jsonbb::Value::from("string").is_string());
    ///
    /// // The boolean `false` is not a string.
    /// assert!(!jsonbb::Value::from(false).is_string());
    /// ```
    pub fn is_string(self) -> bool {
        self.as_ref().is_string()
    }

    /// Returns true if the value is an array. Returns false otherwise.
    pub fn is_array(self) -> bool {
        self.as_ref().is_array()
    }

    /// Returns true if the value is an object. Returns false otherwise.
    pub fn is_object(self) -> bool {
        self.as_ref().is_object()
    }

    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Index into a JSON array or object.
    ///
    /// A string index can be used to access a value in an object,
    /// and a usize index can be used to access an element of an array.
    ///
    /// # Example
    ///
    /// ```
    /// let object: jsonbb::Value = r#"{"a": 1, "b": 2}"#.parse().unwrap();
    /// assert_eq!(object.get("a").unwrap().to_string(), "1");
    /// assert!(object.get("c").is_none());
    /// assert!(object.get(0).is_none());
    ///
    /// let array: jsonbb::Value = r#"["a", "b"]"#.parse().unwrap();
    /// assert_eq!(array.get(0).unwrap().to_string(), "\"a\"");
    /// assert!(array.get(2).is_none());
    /// assert!(array.get("a").is_none());
    /// ```
    pub fn get(&self, index: impl Index) -> Option<ValueRef<'_>> {
        index.index_into(self.as_ref())
    }

    /// Push a value into a JSON array.
    ///
    /// This function is `O(N)` where N is the number of elements in the array.
    ///
    /// # Panics
    ///
    /// Panics if the value is not an array.
    ///
    /// # Example
    /// ```
    /// let mut array: jsonbb::Value = "[1]".parse().unwrap();
    /// array.array_push(jsonbb::Value::from(()).as_ref());
    /// array.array_push(jsonbb::Value::from(2).as_ref());
    /// array.array_push(jsonbb::Value::from("str").as_ref());
    /// array.array_push(jsonbb::Value::array([]).as_ref());
    /// array.array_push(jsonbb::Value::object([]).as_ref());
    /// assert_eq!(array.to_string(), r#"[1,null,2,"str",[],{}]"#);
    /// ```
    pub fn array_push(&mut self, value: ValueRef<'_>) {
        let len = self.as_array().expect("not array").len();
        // The offset to insert the value.
        let offset = self.buffer.len() - 4 - 4 - 4 - 4 * len;
        let mut buffer = std::mem::take(&mut self.buffer).into_vec();
        // reserve space for the value + its entry
        buffer.reserve_exact(value.capacity() + 4);
        // remove tailing (len, size, entry)
        buffer.truncate(buffer.len() - 12);
        // insert the value
        buffer.splice(offset..offset, value.as_slice().iter().copied());
        // push the entry
        buffer.put_u32_ne(value.make_entry(offset).0);
        // push (len, size, entry)
        buffer.put_u32_ne((len + 1) as u32);
        buffer.put_u32_ne((buffer.len() + 4) as u32);
        buffer.put_u32_ne(Entry::array(buffer.len()).0);
        // store the buffer
        self.buffer = buffer.into();
    }

    fn from_builder(capacity: usize, f: impl FnOnce(&mut Builder)) -> Self {
        let mut builder = Builder::with_capacity(capacity);
        f(&mut builder);
        builder.finish()
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

/// # Example
///
/// ```
/// let a: jsonbb::Value = r#"{"a": 1, "b": 2}"#.parse().unwrap();
/// let b: jsonbb::Value = r#"{"b": 2, "a": 1.0}"#.parse().unwrap();
/// assert_eq!(a, b);
/// ```
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compare two JSON values.
///
/// The ordering is defined as follows:
/// <https://www.postgresql.org/docs/current/datatype-json.html#JSON-INDEXING>
///
/// # Example
///
/// ```
/// use jsonbb::Value;
///
/// // Object > Array > Boolean > Number > String > Null
/// let v = ["null", r#""str""#, "-1", "0", "3.14", "false", "true", "[]", "{}"];
/// let v = v.iter().map(|s| s.parse().unwrap()).collect::<Vec<Value>>();
/// for (i, a) in v.iter().enumerate() {
///     for b in v.iter().skip(i + 1) {
///         assert!(a < b);
///     }
/// }
///
/// // Array with n elements > array with n - 1 elements
/// let a: Value = r#"[1, 2, 3]"#.parse().unwrap();
/// let b: Value = r#"[1, 2]"#.parse().unwrap();
/// assert!(a > b);
///
/// // arrays with equal numbers of elements are compared in the order:
/// //  element-1, element-2 ...
/// let a: Value = r#"[1, 2]"#.parse().unwrap();
/// let b: Value = r#"[1, 3]"#.parse().unwrap();
/// assert!(a < b);
///
/// // Object with n pairs > object with n - 1 pairs
/// let a: Value = r#"{"a": 1, "b": 2}"#.parse().unwrap();
/// let b: Value = r#"{"a": 1}"#.parse().unwrap();
/// assert!(a > b);
///
/// // Objects with equal numbers of pairs are compared in the order:
/// //  key-1, value-1, key-2 ...
/// let a: Value = r#"{"a": 1, "b": 2}"#.parse().unwrap();
/// let b: Value = r#"{"a": 2, "b": 1}"#.parse().unwrap();
/// assert!(a < b);
/// ```
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::null()
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

impl From<serde_json::Number> for Value {
    fn from(value: serde_json::Number) -> Self {
        Self::from(&value)
    }
}

impl From<&serde_json::Number> for Value {
    fn from(n: &serde_json::Number) -> Self {
        Self::from_builder(0, |b| b.add_serde_number(n))
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        value.as_ref().into()
    }
}

impl<W: AsMut<Vec<u8>>> Builder<W> {
    /// Adds a serde `Value` recursively to the builder and returns its ptr.
    fn add_serde_value(&mut self, value: &serde_json::Value) {
        match value {
            serde_json::Value::Null => self.add_null(),
            serde_json::Value::Bool(b) => self.add_bool(*b),
            serde_json::Value::Number(n) => self.add_serde_number(n),
            serde_json::Value::String(s) => self.add_string(s),
            serde_json::Value::Array(a) => {
                self.begin_array();
                for v in a.iter() {
                    self.add_serde_value(v);
                }
                self.end_array();
            }
            serde_json::Value::Object(o) => {
                self.begin_object();
                for (k, v) in o.iter() {
                    self.add_string(k);
                    self.add_serde_value(v);
                }
                self.end_object()
            }
        }
    }

    /// Adds a serde `Number`.
    fn add_serde_number(&mut self, n: &serde_json::Number) {
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
}

impl FromStr for Value {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_text(s.as_bytes())
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

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::from(v as u64)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Self::from(v as u64)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Self::from(v as u64)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::from_builder(1 + 8 + 4, |b| b.add_u64(v))
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Self::from(v as u64)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Self::from(v as i64)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Self::from(v as i64)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::from(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::from_builder(1 + 8 + 4, |b| b.add_i64(v))
    }
}

impl From<isize> for Value {
    fn from(v: isize) -> Self {
        Self::from(v as u64)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::from(v as f64)
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

/// Creates a `Value` from bytes of jsonbb encoding.
///
/// If you want to create a `Value` from JSON text, use [`FromStr`] or [`from_text`] instead.
///
/// [`from_text`]: #method.from_text
/// [`FromStr`]: #method.from_str
impl From<&[u8]> for Value {
    fn from(s: &[u8]) -> Self {
        Self::from_bytes(s)
    }
}

impl From<ValueRef<'_>> for Value {
    fn from(v: ValueRef<'_>) -> Self {
        Self::from_builder(v.capacity() + 4, |b| b.add_value(v))
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

    #[test]
    #[should_panic]
    fn from_nan() {
        _ = Value::from(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn from_inf() {
        _ = Value::from(f64::INFINITY);
    }

    #[test]
    #[should_panic]
    fn from_neg_inf() {
        _ = Value::from(f64::NEG_INFINITY);
    }
}
