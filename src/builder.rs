use super::*;
use bytes::BufMut;
use smallvec::SmallVec;

/// A builder for JSON values.
pub struct Builder<W = Vec<u8>> {
    /// The buffer to write to.
    buffer: W,
    /// A stack of pointers.
    ///
    /// Smallvec is used to avoid heap allocation for single value.
    pointers: SmallVec<[Ptr; 1]>,
    /// A stack of (position, number of pointers) pairs when the array/object starts.
    container_starts: Vec<(usize, usize)>,
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
        self.pointers.push(Ptr {
            offset: 0,
            entry: Entry::null(),
        });
    }

    /// Adds a boolean value to the builder.
    pub fn add_bool(&mut self, v: bool) {
        self.pointers.push(Ptr {
            offset: 0,
            entry: if v { Entry::true_() } else { Entry::false_() },
        });
    }

    /// Adds an u64 value to the builder.
    pub fn add_u64(&mut self, v: u64) {
        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        buffer.push(NUMBER_U64);
        buffer.put_u64_ne(v);
        self.pointers.push(Ptr {
            offset,
            entry: Entry::number(),
        });
    }

    /// Adds an i64 value to the builder.
    pub fn add_i64(&mut self, v: i64) {
        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        buffer.push(NUMBER_I64);
        buffer.put_i64_ne(v);
        self.pointers.push(Ptr {
            offset,
            entry: Entry::number(),
        });
    }

    /// Adds an f64 value to the builder.
    pub fn add_f64(&mut self, v: f64) {
        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        buffer.push(NUMBER_F64);
        buffer.put_f64_ne(v);
        self.pointers.push(Ptr {
            offset,
            entry: Entry::number(),
        });
    }

    /// Adds a string value to the builder.
    pub fn add_string(&mut self, v: &str) {
        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        buffer.put_u32_ne(v.len() as u32);
        buffer.put_slice(v.as_bytes());
        self.pointers.push(Ptr {
            offset,
            entry: Entry::string(),
        });
    }

    /// Begins an array.
    pub fn begin_array(&mut self) {
        let buffer = self.buffer.as_mut();
        self.container_starts
            .push((buffer.len(), self.pointers.len()));
    }

    /// Finishes an array.
    pub fn finish_array(&mut self) {
        let buffer = self.buffer.as_mut();
        let (start, npointer) = self.container_starts.pop().unwrap();
        let offset = buffer.len();
        let payload_size = offset - start;
        let len = self.pointers.len() - npointer;
        buffer.reserve(4 + 4 + 4 * len);
        buffer.put_u32_ne(len as u32);
        buffer.put_u32_ne(payload_size as u32);
        for value in self.pointers.drain(npointer..) {
            buffer.put_u32_ne(value.to_entry(offset));
        }
        self.pointers.push(Ptr {
            offset,
            entry: Entry::array(),
        });
    }

    /// Begins an object.
    pub fn begin_object(&mut self) {
        let buffer = self.buffer.as_mut();
        self.container_starts
            .push((buffer.len(), self.pointers.len()));
    }

    /// Finishes an object.
    pub fn finish_object<'b>(&mut self) {
        let buffer = self.buffer.as_mut();
        let (start, npointer) = self.container_starts.pop().unwrap();
        let offset = buffer.len();
        let payload_size = offset - start;
        let len = (self.pointers.len() - npointer) / 2;
        buffer.reserve(4 + 4 + 8 * len);
        buffer.put_u32_ne(len as u32);
        buffer.put_u32_ne(payload_size as u32);
        for ptr in self.pointers.drain(npointer..) {
            buffer.put_u32_ne(ptr.to_entry(offset));
        }
        // sort entries by key
        let entries = unsafe {
            std::slice::from_raw_parts_mut(
                buffer.as_ptr().add(offset + 8) as *mut (Entry, Entry),
                len,
            )
        };
        let base = unsafe { buffer.as_ptr().add(offset) };
        entries.sort_unstable_by_key(|(k, _)| unsafe {
            ValueRef::from_raw(base, *k)
                .as_str()
                .expect("key must be string")
        });

        self.pointers.push(Ptr {
            offset,
            entry: Entry::object(),
        });
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
                let offset = buffer.len() + a.elements_len();
                buffer.extend_from_slice(a.as_slice());
                self.pointers.push(Ptr {
                    offset,
                    entry: Entry::array(),
                });
            }
            ValueRef::Object(o) => {
                let buffer = self.buffer.as_mut();
                let offset = buffer.len() + o.elements_len();
                buffer.extend_from_slice(o.as_slice());
                self.pointers.push(Ptr {
                    offset,
                    entry: Entry::object(),
                });
            }
        }
    }

    /// Finishes building.
    fn finish_internal(mut self) -> W {
        assert_eq!(self.pointers.len(), 1, "expected single root value");
        assert!(self.container_starts.is_empty(), "unfinished container");
        let buffer = self.buffer.as_mut();
        let offset = buffer.len();
        let ptr = self.pointers.pop().unwrap();
        buffer.put_u32_ne(ptr.to_entry(offset));
        self.buffer
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

/// A pointer to a pushed node.
struct Ptr {
    offset: usize,
    entry: Entry,
}

impl Ptr {
    fn to_entry(&self, self_offset: usize) -> u32 {
        self.entry.with_offset(self_offset - self.offset).as_u32()
    }
}
