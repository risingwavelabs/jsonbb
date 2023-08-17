use super::*;
use std::{fmt, str::FromStr};

pub struct Array {
    pub(crate) buffer: Box<[u8]>,
    /// The id of the root array element.
    pub(crate) id: Id,
    pub(crate) len: u32,
}

impl Array {
    pub fn get(&self, index: usize) -> Option<ValueRef<'_>> {
        self.as_ref().get(index)
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = ValueRef<'_>> + '_ {
        self.as_ref().iter()
    }

    pub fn dump(&self) -> String {
        dump(&self.buffer)
    }

    fn as_ref(&self) -> ArrayRef<'_> {
        ArrayRef {
            buffer: &self.buffer,
            id: self.id,
            len: self.len,
        }
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

/// Parse a JSON array into an `Array`.
impl FromStr for Array {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: directly parse into the builder
        let serde_value = serde_json::Value::from_str(s)?;
        let value = Value::from(serde_value);
        let len = value.as_array().expect("not an array").len();
        Ok(Array {
            buffer: value.buffer,
            id: value.id,
            len: len as u32,
        })
    }
}
