use super::*;
use std::fmt;

pub struct Array {
    buffer: Box<[u8]>,
    /// The id of the root array element.
    id: Id,
    len: u32,
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
