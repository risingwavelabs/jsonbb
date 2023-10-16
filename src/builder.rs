use super::*;
use bytes::BufMut;

/// A builder for JSON values.
#[derive(Debug)]
pub struct Builder<'a> {
    buffer: &'a mut Vec<u8>,
}

impl<'a> Builder<'a> {
    /// Creates a new [`Builder`].
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Builder { buffer }
    }

    /// Adds a null value to the builder and returns its Ptr.
    pub fn add_null(&mut self) -> Ptr {
        Ptr {
            offset: 0,
            entry: Entry::null(),
        }
    }

    /// Adds a boolean value to the builder and returns its Ptr.
    pub fn add_bool(&mut self, v: bool) -> Ptr {
        Ptr {
            offset: 0,
            entry: if v { Entry::true_() } else { Entry::false_() },
        }
    }

    /// Adds an u64 value to the builder and returns its Ptr.
    pub fn add_u64(&mut self, v: u64) -> Ptr {
        let offset = self.buffer.len();
        self.buffer.push(NUMBER_U64);
        self.buffer.put_u64_ne(v);
        Ptr {
            offset,
            entry: Entry::number(),
        }
    }

    /// Adds an i64 value to the builder and returns its Ptr.
    pub fn add_i64(&mut self, v: i64) -> Ptr {
        let offset = self.buffer.len();
        self.buffer.push(NUMBER_I64);
        self.buffer.put_i64_ne(v);
        Ptr {
            offset,
            entry: Entry::number(),
        }
    }

    /// Adds an f64 value to the builder and returns its Ptr.
    pub fn add_f64(&mut self, v: f64) -> Ptr {
        let offset = self.buffer.len();
        self.buffer.push(NUMBER_F64);
        self.buffer.put_f64_ne(v);
        Ptr {
            offset,
            entry: Entry::number(),
        }
    }

    /// Adds a string value to the builder and returns its Ptr.
    pub fn add_string(&mut self, v: &str) -> Ptr {
        let offset = self.buffer.len();
        self.buffer.put_u32_ne(v.len() as u32);
        self.buffer.put_slice(v.as_bytes());
        Ptr {
            offset,
            entry: Entry::string(),
        }
    }

    /// Adds an array value to the builder and returns its Ptr.
    pub fn add_array(&mut self, start: usize, values: &[Ptr]) -> Ptr {
        let offset = self.buffer.len();
        let payload_size = offset - start;
        self.buffer.reserve(4 + 4 + 4 * values.len());
        self.buffer.put_u32_ne(values.len() as u32);
        self.buffer.put_u32_ne(payload_size as u32);
        for value in values {
            self.buffer.put_u32_ne(value.to_entry(offset));
        }
        Ptr {
            offset,
            entry: Entry::array(),
        }
    }

    /// Adds an object value to the builder and returns its Ptr.
    pub fn add_object<'b>(
        &mut self,
        start: usize,
        kvs: impl ExactSizeIterator<Item = (Ptr, Ptr)>,
    ) -> Ptr {
        let offset = self.buffer.len();
        let payload_size = offset - start;
        self.buffer.reserve(4 + 4 + 8 * kvs.len());
        self.buffer.put_u32_ne(kvs.len() as u32);
        self.buffer.put_u32_ne(payload_size as u32);
        for (kptr, vptr) in kvs {
            self.buffer.put_u32_ne(kptr.to_entry(offset));
            self.buffer.put_u32_ne(vptr.to_entry(offset));
        }
        // TODO: sort entries by key
        Ptr {
            offset,
            entry: Entry::object(),
        }
    }

    /// Adds a value to the builder and returns its Ptr.
    pub fn add_value_ref(&mut self, value: ValueRef<'_>) -> Ptr {
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
                let offset = self.buffer.len() + a.elements_len();
                self.buffer.extend_from_slice(a.as_slice());
                Ptr {
                    offset,
                    entry: Entry::array(),
                }
            }
            ValueRef::Object(o) => {
                let offset = self.buffer.len() + o.elements_len();
                self.buffer.extend_from_slice(o.as_slice());
                Ptr {
                    offset,
                    entry: Entry::object(),
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Finishes building.
    pub fn finish(self, ptr: Ptr) {
        let offset = self.buffer.len();
        self.buffer.put_u32_ne(ptr.to_entry(offset));
    }
}

pub struct Ptr {
    offset: usize,
    entry: Entry,
}

impl Ptr {
    fn to_entry(&self, self_offset: usize) -> u32 {
        self.entry.with_offset(self_offset - self.offset).as_u32()
    }
}
