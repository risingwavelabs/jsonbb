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
use bytes::{Buf, BufMut};
use smallvec::SmallVec;
use std::fmt::{self, Debug, Display};

/// A builder for JSON values.
pub struct Builder<W = Vec<u8>> {
    /// The buffer to write to.
    buffer: W,
    /// A stack of entries.
    ///
    /// Smallvec is used to avoid heap allocation for single value.
    pointers: SmallVec<[Entry; 1]>,
    /// A stack of (position, number of pointers) pairs when the array/object starts.
    container_starts: Vec<(usize, usize)>,
}

impl<W> Debug for Builder<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("JsonbbBuilder").finish()
    }
}

impl Default for Builder<Vec<u8>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder<Vec<u8>> {
    /// Creates a new [`Builder`].
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new [`Builder`] with capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Builder {
            buffer: Vec::with_capacity(capacity),
            pointers: SmallVec::new(),
            container_starts: vec![],
        }
    }
}

impl<'a> Builder<&'a mut Vec<u8>> {
    /// Creates a new [`Builder`].
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Builder {
            buffer,
            pointers: SmallVec::new(),
            container_starts: vec![],
        }
    }
}

impl<W: AsMut<Vec<u8>>> Builder<W> {
    /// Adds a null value to the builder.
    pub fn add_null(&mut self) {
        self.pointers.push(Entry::null());
    }

    /// Adds a boolean value to the builder.
    pub fn add_bool(&mut self, v: bool) {
        let entry = if v { Entry::true_() } else { Entry::false_() };
        self.pointers.push(entry);
    }

    /// Adds an u64 value to the builder.
    pub fn add_u64(&mut self, v: u64) {
        let offset = self.offset();
        self.pointers.push(Entry::number(offset));
        let buffer = self.buffer.as_mut();
        buffer.push(NUMBER_U64);
        buffer.put_u64_ne(v);
    }

    /// Adds an i64 value to the builder.
    pub fn add_i64(&mut self, v: i64) {
        if v >= 0 {
            return self.add_u64(v as u64);
        }
        let offset = self.offset();
        self.pointers.push(Entry::number(offset));
        let buffer = self.buffer.as_mut();
        buffer.push(NUMBER_I64);
        buffer.put_i64_ne(v);
    }

    /// Adds an f64 value to the builder.
    pub fn add_f64(&mut self, v: f64) {
        assert!(
            !v.is_nan() && !v.is_infinite(),
            "Infinite or NaN values are not JSON numbers"
        );
        let offset = self.offset();
        self.pointers.push(Entry::number(offset));
        let buffer = self.buffer.as_mut();
        buffer.push(NUMBER_F64);
        buffer.put_f64_ne(v);
    }

    /// Adds a string value to the builder.
    pub fn add_string(&mut self, v: &str) {
        let offset = self.offset();
        self.pointers.push(Entry::string(offset));
        let buffer = self.buffer.as_mut();
        buffer.put_u32_ne(v.len().try_into().expect("string too long"));
        buffer.put_slice(v.as_bytes());
    }

    /// Adds a string value that displays the given value to the builder.
    pub fn display(&mut self, v: impl Display) {
        use std::io::Write;

        let offset = self.offset();
        self.pointers.push(Entry::string(offset));

        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        buffer.put_u32_ne(0); // placeholder for length
        write!(buffer, "{}", v).unwrap();

        // update length
        let len = buffer.len() - offset - 4;
        (&mut buffer[offset..]).put_u32_ne(len.try_into().expect("string too long"));
    }

    /// Begins an array.
    ///
    /// The caller then needs to push the elements and call [`end_array`] to finish the array.
    ///
    /// [`end_array`]: #method.end_array
    pub fn begin_array(&mut self) {
        let buffer = self.buffer.as_mut();
        self.container_starts
            .push((buffer.len(), self.pointers.len()));
    }

    /// Ends an array.
    pub fn end_array(&mut self) {
        let buffer = self.buffer.as_mut();
        let (start, npointer) = self.container_starts.pop().unwrap();
        let len = self.pointers.len() - npointer;
        buffer.reserve(4 * len + 4 + 4);
        for entry in self.pointers.drain(npointer..) {
            buffer.put_u32_ne(entry.0);
        }
        buffer.put_u32_ne(len as u32);
        buffer.put_u32_ne((buffer.len() - start + 4) as u32);

        let offset = self.offset();
        self.pointers.push(Entry::array(offset));
    }

    /// Begins an object.
    ///
    /// The caller then needs to push the keys and values in the following order:
    /// ```text
    /// key-1, value-1, key-2, value-2 ...
    /// ```
    /// where each key must be a string.
    ///
    /// Keys are allowed to be duplicated, but the last value will be used.
    ///
    /// Finally [`end_object`] must be called to finish the object.
    ///
    /// [`end_object`]: #method.end_object
    pub fn begin_object(&mut self) {
        let buffer = self.buffer.as_mut();
        self.container_starts
            .push((buffer.len(), self.pointers.len()));
    }

    /// Ends an object.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - there is an odd number of entries pushed since the paired [`begin_object`].
    /// - any key is not a string.
    ///
    /// [`begin_object`]: #method.begin_object
    pub fn end_object(&mut self) {
        let buffer = self.buffer.as_mut();
        let (start, npointer) = self.container_starts.pop().unwrap();
        assert!(
            (self.pointers.len() - npointer) % 2 == 0,
            "expected even number of entries"
        );
        let len = (self.pointers.len() - npointer) / 2;

        // sort entries by key
        // TODO: use `as_chunks_mut` when stabilized
        let entries = unsafe {
            std::slice::from_raw_parts_mut(
                self.pointers
                    .as_mut_ptr()
                    .add(npointer)
                    .cast::<(Entry, Entry)>(),
                len,
            )
        };
        for (k, _) in entries.iter() {
            assert!(k.is_string(), "key must be string");
        }
        let entry_to_str = |entry: Entry| {
            // Performance tip: this closure is in hot path, so we use `unsafe` to avoid bound check.
            // SAFETY: the string is pushed by us, so it's valid UTF-8 and the range is valid.
            let offset = start + entry.offset();
            unsafe {
                let len = buffer.as_ptr().add(offset).cast::<u32>().read_unaligned() as usize;
                std::str::from_utf8_unchecked(buffer.get_unchecked(offset + 4..offset + 4 + len))
            }
        };
        entries.sort_by_key(|(k, _)| entry_to_str(*k));

        // deduplicate keys
        let mut prev_key = None;
        let mut unique_len = 0;
        for i in 0..len {
            let key = entry_to_str(entries[i].0);
            if prev_key != Some(key) {
                prev_key = Some(key);
                entries[unique_len] = entries[i];
                unique_len += 1;
            } else {
                entries[unique_len - 1] = entries[i];
            }
        }

        // write entries to buffer
        buffer.reserve(8 * unique_len + 4 + 4);
        for (kentry, ventry) in &entries[..unique_len] {
            buffer.put_u32_ne(kentry.0);
            buffer.put_u32_ne(ventry.0);
        }
        buffer.put_u32_ne(unique_len as u32);
        buffer.put_u32_ne((buffer.len() - start + 4) as u32);

        let offset = self.offset();
        self.pointers.truncate(npointer);
        self.pointers.push(Entry::object(offset));
    }

    /// Adds a JSON value to the builder.
    pub fn add_value(&mut self, value: ValueRef<'_>) {
        match value {
            ValueRef::Null => self.add_null(),
            ValueRef::Bool(b) => self.add_bool(b),
            ValueRef::Number(n) => {
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
            ValueRef::String(s) => self.add_string(s),
            ValueRef::Array(a) => {
                let buffer = self.buffer.as_mut();
                buffer.extend_from_slice(a.as_slice());
                let offset = self.offset();
                self.pointers.push(Entry::array(offset));
            }
            ValueRef::Object(o) => {
                let buffer = self.buffer.as_mut();
                buffer.extend_from_slice(o.as_slice());
                let offset = self.offset();
                self.pointers.push(Entry::object(offset));
            }
        }
    }

    /// Finishes building.
    fn finish_internal(mut self) -> W {
        assert_eq!(self.pointers.len(), 1, "expected single root value");
        assert!(self.container_starts.is_empty(), "unfinished container");
        let buffer = self.buffer.as_mut();
        let entry = self.pointers.pop().unwrap();
        buffer.put_u32_ne(entry.0);
        self.buffer
    }

    /// Get the current offset from the array/object start.
    fn offset(&mut self) -> usize {
        self.buffer.as_mut().len() - self.container_starts.last().map_or(0, |&(o, _)| o)
    }

    /// Pops the last value.
    pub fn pop(&mut self) {
        let entry = self.pointers.pop().unwrap();
        if entry == Entry::null() || entry == Entry::false_() || entry == Entry::true_() {
            // no payload
            return;
        }
        let buffer = self.buffer.as_mut();
        let new_len = entry.offset() + self.container_starts.last().map_or(0, |&(o, _)| o);
        buffer.truncate(new_len);
        if entry.is_array() || entry.is_object() {
            let len = (&buffer[new_len - 4..]).get_u32_ne() as usize;
            buffer.truncate(new_len - len);
        }
    }

    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&mut self) -> usize {
        self.buffer.as_mut().capacity()
    }
}

impl Builder<Vec<u8>> {
    /// Finishes building.
    pub fn finish(self) -> Value {
        Value {
            buffer: self.finish_internal().into(),
        }
    }
}

impl<'a> Builder<&'a mut Vec<u8>> {
    /// Finishes building.
    pub fn finish(self) {
        self.finish_internal();
    }
}

#[cfg(test)]
mod tests {
    use crate::{Builder, Value};

    #[test]
    fn unique_key() {
        let value: Value = r#"{"a":1,"b":2,"a":3}"#.parse().unwrap();
        assert_eq!(value.to_string(), r#"{"a":3,"b":2}"#);
    }

    #[test]
    fn pop() {
        let mut builder = Builder::<Vec<u8>>::new();
        builder.begin_array();
        builder.add_u64(1);
        builder.add_string("2");
        builder.add_null();
        builder.begin_array();
        builder.add_null();
        builder.end_array();
        builder.pop();
        builder.pop();
        builder.pop();
        builder.add_u64(4);
        builder.end_array();
        let value = builder.finish();
        assert_eq!(value.to_string(), "[1,4]");
    }
}
