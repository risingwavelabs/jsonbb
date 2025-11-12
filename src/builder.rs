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
    /// A stack of [position, number of pointers] pairs when the array/object starts.
    container_starts: Vec<[usize; 2]>,
}

/// A checkpoint of the builder state.
#[derive(Debug, Clone)]
pub struct CheckPoint {
    buffer_length: usize,
    pointer_length: usize,
    container_starts_length: usize,
    checksum: u32,
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

impl<W: Clone> Clone for Builder<W> {
    fn clone(&self) -> Self {
        Builder {
            buffer: self.buffer.clone(),
            pointers: self.pointers.clone(),
            container_starts: self.container_starts.clone(),
        }
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
        if let Ok(v) = i64::try_from(v) {
            return self.add_i64(v);
        }
        let offset = self.offset();
        self.pointers.push(Entry::number(offset));
        let buffer = self.buffer.as_mut();
        buffer.push(NUMBER_U64);
        buffer.put_u64_ne(v);
    }

    /// Adds an i64 value to the builder.
    pub fn add_i64(&mut self, v: i64) {
        let offset = self.offset();
        self.pointers.push(Entry::number(offset));
        let buffer = self.buffer.as_mut();
        if v == 0 {
            buffer.push(NUMBER_ZERO);
        } else if let Ok(v) = i8::try_from(v) {
            buffer.push(NUMBER_I8);
            buffer.put_i8(v);
        } else if let Ok(v) = i16::try_from(v) {
            buffer.push(NUMBER_I16);
            buffer.put_i16_ne(v);
        } else if let Ok(v) = i32::try_from(v) {
            buffer.push(NUMBER_I32);
            buffer.put_i32_ne(v);
        } else {
            buffer.push(NUMBER_I64);
            buffer.put_i64_ne(v);
        }
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
            .push([buffer.len(), self.pointers.len()]);
    }

    /// Ends an array.
    pub fn end_array(&mut self) {
        let buffer = self.buffer.as_mut();
        let [start, npointer] = self.container_starts.pop().unwrap();
        let len = self.pointers.len() - npointer;
        buffer.reserve(4 * len + 4 + 4);
        for entry in self.pointers.drain(npointer..) {
            buffer.put_slice(entry.as_bytes());
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
            .push([buffer.len(), self.pointers.len()]);
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
        let [start, npointer] = self.container_starts.pop().unwrap();
        assert!(
            (self.pointers.len() - npointer).is_multiple_of(2),
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

        // remove data if there are duplicates
        if unique_len != len {
            let data = &mut buffer[start..];
            // get the index order by offset
            // TODO: reuse buffer to avoid allocation
            let mut indices = (0..unique_len).collect::<Vec<_>>();
            indices.sort_unstable_by_key(|&i| entries[i].0.offset());
            // compact data and update offset
            let mut new_offset = 0;
            for i in indices {
                // get data range
                let (k, v) = &mut entries[i];
                let begin = k.offset();
                let end = if v.is_number() {
                    v.offset() + 1 + number_size(data[v.offset()])
                } else if v.is_string() {
                    v.offset() + 4 + (&data[v.offset()..]).get_u32_ne() as usize
                } else if v.is_array() || v.is_object() {
                    v.offset()
                } else {
                    // null, false, true: no data for value
                    begin + 4 + (&data[begin..]).get_u32_ne() as usize
                };
                // move data and update entry
                if begin != new_offset {
                    // eprintln!("move {:?} to {}", begin..end, new_offset);
                    data.copy_within(begin..end, new_offset);
                    // update entry
                    let sub = begin - new_offset;
                    k.set_offset(new_offset);
                    if v.offset() != 0 {
                        v.set_offset(v.offset() - sub);
                    }
                }
                new_offset += end - begin;
            }
            buffer.truncate(start + new_offset);
        }

        // write entries to buffer
        buffer.reserve(8 * unique_len + 4 + 4);
        for (kentry, ventry) in &entries[..unique_len] {
            buffer.put_slice(kentry.as_bytes());
            buffer.put_slice(ventry.as_bytes());
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
            ValueRef::String(s) => self.add_string(s.as_str()),
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
        buffer.put_slice(entry.as_bytes());
        self.buffer
    }

    /// Get the current offset from the array/object start.
    fn offset(&mut self) -> usize {
        self.buffer.as_mut().len() - self.container_starts.last().map_or(0, |&[o, _]| o)
    }

    /// Pops the last value.
    pub fn pop(&mut self) {
        let entry = self.pointers.pop().unwrap();
        if entry == Entry::null() || entry == Entry::false_() || entry == Entry::true_() {
            // no payload
            return;
        }
        let buffer = self.buffer.as_mut();
        let new_len = entry.offset() + self.container_starts.last().map_or(0, |&[o, _]| o);
        buffer.truncate(new_len);
        if entry.is_array() || entry.is_object() {
            let len = (&buffer[new_len - 4..]).get_u32_ne() as usize;
            buffer.truncate(new_len - len);
        }
    }

    /// Roll back the builder state to the given checkpoint.
    ///
    /// Only data added after the checkpoint will be removed. If the builder
    /// has already popped more data than recorded in the checkpoint, or if
    /// the checkpoint is invalid (checksum mismatch), the rollback fails.
    ///
    /// Returns `true` if the rollback succeeded, or `false` if the checkpoint
    /// is invalid or the builder state is incompatible with the checkpoint.
    pub fn rollback_to(&mut self, checkpoint: &CheckPoint) -> bool {
        let buffer = self.buffer.as_mut();

        if checkpoint.buffer_length > buffer.len()
            || checkpoint.pointer_length > self.pointers.len()
            || checkpoint.container_starts_length > self.container_starts.len()
        {
            return false;
        }
        let checksum = {
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(&buffer[..checkpoint.buffer_length]);
            hasher.update(bytemuck::cast_slice(
                &self.pointers[..checkpoint.pointer_length],
            ));
            hasher.update(bytemuck::cast_slice(
                &self.container_starts[..checkpoint.container_starts_length],
            ));
            hasher.finalize()
        };
        if checksum != checkpoint.checksum {
            return false;
        }

        buffer.truncate(checkpoint.buffer_length);
        self.pointers.truncate(checkpoint.pointer_length);
        self.container_starts
            .truncate(checkpoint.container_starts_length);
        true
    }
}

impl<W: AsRef<[u8]>> Builder<W> {
    /// Creates a checkpoint of the current state.
    ///
    /// The checkpoint records the current buffer length, stack lengths, and a CRC32
    /// checksum of those portions so a later rollback can validate that no data
    /// has been modified since the checkpoint was created.
    pub fn checkpoint(&self) -> CheckPoint {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(self.buffer.as_ref());
        hasher.update(bytemuck::cast_slice(&self.pointers));
        hasher.update(bytemuck::cast_slice(&self.container_starts));
        CheckPoint {
            buffer_length: self.buffer.as_ref().len(),
            pointer_length: self.pointers.len(),
            container_starts_length: self.container_starts.len(),
            checksum: hasher.finalize(),
        }
    }
}

impl Builder<Vec<u8>> {
    /// Returns the capacity of the internal buffer, in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Finishes building.
    pub fn finish(self) -> Value {
        Value {
            buffer: self.finish_internal().into(),
        }
    }
}

impl Builder<&mut Vec<u8>> {
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
        let value: Value =
            r#"{"a":1,"b":2,"c":3,"d":4,"e":5,"e":{},"d":[0],"c":"c","b":1,"a":null}"#
                .parse()
                .unwrap();
        assert_eq!(
            value.to_string(),
            r#"{"a":null,"b":1,"c":"c","d":[0],"e":{}}"#
        );
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

    #[test]
    fn rollback() {
        let mut builder = Builder::<Vec<u8>>::new();
        builder.begin_array();
        builder.add_u64(1);
        let checkpoint = builder.checkpoint();
        builder.add_string("2");
        builder.add_null();
        builder.begin_array();
        builder.add_null();
        builder.end_array();
        assert!(builder.rollback_to(&checkpoint));
        builder.add_u64(4);
        builder.end_array();
        let value = builder.finish();
        assert_eq!(value.to_string(), "[1,4]");
    }

    #[test]
    fn rollback_invalid() {
        let mut builder = Builder::<Vec<u8>>::new();
        builder.begin_array();
        builder.add_u64(1);
        let checkpoint = builder.checkpoint();
        builder.pop();
        builder.add_u64(2);
        assert!(!builder.rollback_to(&checkpoint));
    }
}
