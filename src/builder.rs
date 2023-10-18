use super::*;
use bytes::BufMut;
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
            buffer.put_u32_ne(entry.as_u32());
        }
        buffer.put_u32_ne(len as u32);
        buffer.put_u32_ne((buffer.len() - start + 4) as u32);

        let offset = self.offset();
        self.pointers.push(Entry::array(offset));
    }

    /// Begins an object.
    pub fn begin_object(&mut self) {
        let buffer = self.buffer.as_mut();
        self.container_starts
            .push((buffer.len(), self.pointers.len()));
    }

    /// Ends an object.
    pub fn end_object(&mut self) {
        let buffer = self.buffer.as_mut();
        let (start, npointer) = self.container_starts.pop().unwrap();
        let base = buffer.len();
        let len = (self.pointers.len() - npointer) / 2;
        buffer.reserve(8 * len + 4 + 4);
        for entry in self.pointers.drain(npointer..) {
            buffer.put_u32_ne(entry.as_u32());
        }
        buffer.put_u32_ne(len as u32);
        buffer.put_u32_ne((buffer.len() - start + 4) as u32);

        // sort entries by key
        let entries = unsafe {
            std::slice::from_raw_parts_mut(buffer.as_ptr().add(base) as *mut (Entry, Entry), len)
        };
        let data = &buffer[start..];
        entries.sort_unstable_by_key(|(k, _)| {
            ValueRef::from_slice(data, *k)
                .as_str()
                .expect("key must be string")
        });

        let offset = self.offset();
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
        buffer.put_u32_ne(entry.as_u32());
        self.buffer
    }

    /// Get the current offset from the array/object start.
    fn offset(&mut self) -> usize {
        self.buffer.as_mut().len() - self.container_starts.last().map_or(0, |&(o, _)| o)
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
