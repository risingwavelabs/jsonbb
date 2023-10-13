use super::*;
use bytes::BufMut;

/// A builder for JSON values.
#[derive(Default, Debug)]
pub struct Builder {
    buffer: Vec<u8>,
    // string_ids: HashMap<String, Id>,
}

impl Builder {
    /// Creates a new [`Builder`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Builder`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            // string_ids: HashMap::new(),
        }
    }

    /// Returns the ID that will be assigned to the next value.
    fn next_id(&self) -> Id {
        Id(self.buffer.len() as u32)
    }

    /// Adds a null value to the builder and returns its ID.
    pub fn add_null(&mut self) -> Id {
        Id::NULL
    }

    /// Adds a boolean value to the builder and returns its ID.
    pub fn add_bool(&mut self, v: bool) -> Id {
        if v {
            Id::TRUE
        } else {
            Id::FALSE
        }
    }

    /// Adds an u64 value to the builder and returns its ID.
    pub fn add_u64(&mut self, v: u64) -> Id {
        let id = self.next_id();
        self.buffer.push(TAG_U64);
        self.buffer.put_u64_le(v);
        id
    }

    /// Adds an i64 value to the builder and returns its ID.
    pub fn add_i64(&mut self, v: i64) -> Id {
        let id = self.next_id();
        self.buffer.push(TAG_I64);
        self.buffer.put_i64_le(v);
        id
    }

    /// Adds an f64 value to the builder and returns its ID.
    pub fn add_f64(&mut self, v: f64) -> Id {
        let id = self.next_id();
        self.buffer.push(TAG_F64);
        self.buffer.put_f64_le(v);
        id
    }

    /// Adds a string value to the builder and returns its ID.
    pub fn add_string(&mut self, v: &str) -> Id {
        // if let Some(id) = self.string_ids.get(v) {
        //     return *id;
        // }
        let id = self.next_id();
        // self.string_ids.insert(v.into(), id);
        self.buffer.push(TAG_STRING);
        self.buffer.put_u32_le(v.len() as u32);
        self.buffer.put_slice(v.as_bytes());
        id
    }

    /// Adds an array value to the builder and returns its ID.
    pub fn add_array(&mut self, values: &[Id]) -> Id {
        let id = self.next_id();
        self.buffer.reserve(1 + Id::SIZE * (1 + values.len()));
        self.buffer.push(TAG_ARRAY);
        self.buffer.put_u32_le(values.len() as u32);
        for value in values {
            self.buffer.put_u32_le(value.0);
        }
        id
    }

    /// Adds an object value to the builder and returns its ID.
    pub fn add_object<'b>(&mut self, kvs: impl Iterator<Item = (&'b str, Id)>) -> Id {
        // add keys
        let mut last_key = None;
        let mut ids = Vec::with_capacity(kvs.size_hint().0);
        for (k, v) in kvs {
            assert!(Some(k) > last_key, "keys must be ordered");
            last_key = Some(k);
            let kid = self.add_string(k);
            ids.push((kid, v));
        }
        self.add_object_ids(&ids)
    }

    /// Adds an object value to the builder and returns its ID.
    pub(crate) fn add_object_ids<'b>(&mut self, kvs: &[(Id, Id)]) -> Id {
        // add object
        let id = self.next_id();
        self.buffer.reserve(1 + Id::SIZE * (1 + kvs.len() * 2));
        self.buffer.push(TAG_OBJECT);
        self.buffer.put_u32_le(kvs.len() as u32);
        for (k, v) in kvs {
            self.buffer.put_u32_le(k.0);
            self.buffer.put_u32_le(v.0);
        }
        id
    }

    /// Adds a value to the builder and returns its ID.
    pub fn add_value_ref(&mut self, value: ValueRef<'_>) -> Id {
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
                let ids = a.iter().map(|v| self.add_value_ref(v)).collect::<Vec<_>>();
                self.add_array(&ids)
            }
            ValueRef::Object(o) => {
                let kvs = o
                    .iter()
                    .map(|(k, v)| (k, self.add_value_ref(v)))
                    .collect::<Vec<_>>();
                self.add_object(kvs.into_iter())
            }
        }
    }

    /// Finishes building and returns the [`Value`].
    pub fn finish(self, id: Id) -> Value {
        Value {
            buffer: self.buffer.into(),
            id,
        }
    }

    /// Finishes building and returns the [`Array`].
    pub fn finish_array(mut self, ids: &[Id]) -> Array {
        let id = self.add_array(ids);
        Array {
            buffer: self.buffer.into(),
            id,
            len: ids.len() as u32,
        }
    }
}
