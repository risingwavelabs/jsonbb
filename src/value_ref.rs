use std::hash::{Hash, Hasher};

use super::*;
use bytes::Buf;
use serde_json::Number;

/// A reference to a JSON value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueRef<'a> {
    // NOTE: Order matters!
    // we follow postgresql's order:
    //  Object > Array > Boolean > Number > String > Null
    /// Represents a JSON null value.
    Null,
    /// Represents a JSON string.
    String(&'a str),
    /// Represents a JSON number.
    Number(NumberRef<'a>),
    /// Represents a JSON boolean.
    Bool(bool),
    /// Represents a JSON array.
    Array(ArrayRef<'a>),
    /// Represents a JSON object.
    Object(ObjectRef<'a>),
}

impl<'a> ValueRef<'a> {
    /// Creates a `ValueRef` from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> ValueRef<'_> {
        let entry = Entry((&bytes[bytes.len() - 4..]).get_u32_ne());
        ValueRef::from_slice(bytes, entry)
    }

    /// If the value is `null`, returns `()`. Returns `None` otherwise.
    pub fn as_null(self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    /// If the value is a boolean, returns the associated bool. Returns `None` otherwise.
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// If the value is a number, returns the associated number. Returns `None` otherwise.
    pub fn as_number(self) -> Option<NumberRef<'a>> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// If the value is an integer, returns the associated u64. Returns `None` otherwise.
    pub fn as_u64(self) -> Option<u64> {
        match self {
            Self::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the value is an integer, returns the associated i64. Returns `None` otherwise.
    pub fn as_i64(self) -> Option<i64> {
        match self {
            Self::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the value is a float, returns the associated f64. Returns `None` otherwise.
    pub fn as_f64(self) -> Option<f64> {
        match self {
            Self::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    /// If the value is a string, returns the associated str. Returns `None` otherwise.
    pub fn as_str(self) -> Option<&'a str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// If the value is an array, returns the associated array. Returns `None` otherwise.
    pub fn as_array(self) -> Option<ArrayRef<'a>> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// If the value is an object, returns the associated map. Returns `None` otherwise.
    pub fn as_object(self) -> Option<ObjectRef<'a>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Creates owned `Value` from `ValueRef`.
    pub fn to_owned(self) -> Value {
        self.into()
    }

    pub(crate) fn from_slice(data: &'a [u8], entry: Entry) -> Self {
        match entry.tag() {
            Entry::NULL_TAG => Self::Null,
            Entry::FALSE_TAG => Self::Bool(false),
            Entry::TRUE_TAG => Self::Bool(true),
            Entry::NUMBER_TAG => {
                let ptr = entry.offset();
                let data = &data[ptr..ptr + 9];
                Self::Number(NumberRef { data })
            }
            Entry::STRING_TAG => {
                let ptr = entry.offset();
                let len = (&data[ptr..]).get_u32_ne() as usize;
                // SAFETY: we don't check for utf8 validity because it's expensive
                let payload =
                    unsafe { std::str::from_utf8_unchecked(&data[ptr + 4..ptr + 4 + len]) };
                Self::String(payload)
            }
            Entry::ARRAY_TAG => {
                let ptr = entry.offset();
                Self::Array(ArrayRef::from_slice(data, ptr))
            }
            Entry::OBJECT_TAG => {
                let ptr = entry.offset();
                Self::Object(ObjectRef::from_slice(data, ptr))
            }
            _ => panic!("invalid entry"),
        }
    }

    /// Returns the entire value as a slice.
    pub(crate) fn as_slice(self) -> &'a [u8] {
        match self {
            Self::Null => &[],
            Self::Bool(_) => &[],
            Self::Number(n) => n.data,
            Self::String(s) => unsafe {
                // SAFETY: include the 4 bytes for the length
                std::slice::from_raw_parts(s.as_ptr().sub(4), s.len() + 4)
            },
            Self::Array(a) => a.as_slice(),
            Self::Object(o) => o.as_slice(),
        }
    }

    /// Makes an entry from the value.
    pub(crate) fn make_entry(self, offset: usize) -> Entry {
        match self {
            Self::Null => Entry::null(),
            Self::Bool(b) => Entry::bool(b),
            Self::Number(_) => Entry::number(offset),
            Self::String(_) => Entry::string(offset),
            Self::Array(_) => Entry::array(offset),
            Self::Object(_) => Entry::object(offset),
        }
    }

    /// Returns the capacity to store this value, in bytes.
    pub fn capacity(self) -> usize {
        self.as_slice().len()
    }

    /// Index into a JSON array or object.
    /// A string index can be used to access a value in an object,
    /// and a usize index can be used to access an element of an array.
    pub fn get(self, index: impl Index) -> Option<ValueRef<'a>> {
        index.index_into(self)
    }
}

impl fmt::Debug for ValueRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool(b) => b.fmt(f),
            Self::Number(n) => n.fmt(f),
            Self::String(s) => s.fmt(f),
            Self::Array(a) => a.fmt(f),
            Self::Object(o) => o.fmt(f),
        }
    }
}

/// Display a JSON value as a string.
impl fmt::Display for ValueRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serialize_in_json(self, f)
    }
}

/// Build a `serde_json::Value` from a jsonbb node.
impl From<ValueRef<'_>> for serde_json::Value {
    fn from(value: ValueRef<'_>) -> Self {
        match value {
            ValueRef::Null => Self::Null,
            ValueRef::Bool(b) => Self::Bool(b),
            ValueRef::Number(n) => Self::Number(n.to_number()),
            ValueRef::String(s) => Self::String(s.to_owned()),
            ValueRef::Array(a) => Self::Array(a.iter().map(Self::from).collect()),
            ValueRef::Object(o) => Self::Object(
                o.iter()
                    .map(|(k, v)| (k.to_owned(), Self::from(v)))
                    .collect(),
            ),
        }
    }
}

/// A reference to a JSON number.
#[derive(Clone, Copy)]
pub struct NumberRef<'a> {
    // # layout
    // | tag | number |
    // |  1  |   8    |
    data: &'a [u8],
}

impl NumberRef<'_> {
    /// Dereferences the number.
    pub(crate) fn to_number(self) -> Number {
        let mut data = self.data;
        match data.get_u8() {
            NUMBER_U64 => Number::from(data.get_u64_ne()),
            NUMBER_I64 => Number::from(data.get_i64_ne()),
            NUMBER_F64 => Number::from_f64(data.get_f64_ne()).unwrap(),
            _ => panic!("invalid number tag"),
        }
    }

    /// If the number is an integer, returns the associated u64. Returns `None` otherwise.
    pub fn as_u64(self) -> Option<u64> {
        self.to_number().as_u64()
    }

    /// If the number is an integer, returns the associated i64. Returns `None` otherwise.
    pub fn as_i64(self) -> Option<i64> {
        self.to_number().as_i64()
    }

    /// If the number is a float, returns the associated f64. Returns `None` otherwise.
    pub fn as_f64(self) -> Option<f64> {
        self.to_number().as_f64()
    }
}

impl fmt::Debug for NumberRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_number().fmt(f)
    }
}

impl fmt::Display for NumberRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_number().fmt(f)
    }
}

impl PartialEq for NumberRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.to_number();
        let b = other.to_number();
        match (a.as_u64(), b.as_u64()) {
            (Some(a), Some(b)) => return a == b,           // a, b > 0
            (Some(_), None) if b.is_i64() => return false, // a >= 0 > b
            (None, Some(_)) if a.is_i64() => return false, // a < 0 <= b
            (None, None) => {
                if let (Some(a), Some(b)) = (a.as_i64(), b.as_i64()) {
                    return a == b; // a, b < 0
                }
            }
            _ => {}
        }
        // either a or b is a float
        let a = a.as_f64().unwrap();
        let b = b.as_f64().unwrap();
        a == b
    }
}

impl Eq for NumberRef<'_> {}

impl PartialOrd for NumberRef<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NumberRef<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.to_number();
        let b = other.to_number();
        match (a.as_u64(), b.as_u64()) {
            (Some(a), Some(b)) => return a.cmp(&b), // a, b > 0
            (Some(_), None) if b.is_i64() => return std::cmp::Ordering::Greater, // a >= 0 > b
            (None, Some(_)) if a.is_i64() => return std::cmp::Ordering::Less, // a < 0 <= b
            (None, None) => {
                if let (Some(a), Some(b)) = (a.as_i64(), b.as_i64()) {
                    return a.cmp(&b); // a, b < 0
                }
            }
            _ => {}
        }
        // either a or b is a float
        let a = a.as_f64().unwrap();
        let b = b.as_f64().unwrap();
        a.partial_cmp(&b).expect("NaN or Inf in JSON number")
    }
}

impl Hash for NumberRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_number().hash(state);
    }
}

/// A reference to a JSON array.
#[derive(Clone, Copy)]
pub struct ArrayRef<'a> {
    // # layout
    //      v---------\
    // | elements | [eptr] x len | len | size |
    // |          |   4 x len    |  4  |  4   |
    // |<----------- data (size) ------------>|^ptr
    data: &'a [u8],
}

impl<'a> ArrayRef<'a> {
    /// Returns the element at the given index, or `None` if the index is out of bounds.
    pub fn get(self, index: usize) -> Option<ValueRef<'a>> {
        let len = self.len();
        if index >= len {
            return None;
        }
        let entry = Entry((&self.data[self.data.len() - 8 - 4 * (len - index)..]).get_u32_ne());
        Some(ValueRef::from_slice(self.data, entry))
    }

    /// Returns the number of elements in the array.
    pub fn len(self) -> usize {
        (&self.data[self.data.len() - 8..]).get_u32_ne() as usize
    }

    /// Returns `true` if the array contains no elements.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the array's elements.
    pub fn iter(self) -> impl ExactSizeIterator<Item = ValueRef<'a>> {
        let len = self.len();
        let mut entries = &self.data[self.data.len() - 8 - 4 * len..];
        (0..len).map(move |_| ValueRef::from_slice(self.data, Entry(entries.get_u32_ne())))
    }

    /// Returns the entire array as a slice.
    pub(crate) fn as_slice(self) -> &'a [u8] {
        self.data
    }

    /// Creates an `ArrayRef` from a slice.
    fn from_slice(data: &'a [u8], end: usize) -> Self {
        let size = (&data[end - 4..end]).get_u32_ne() as usize;
        Self {
            data: &data[end - size..end],
        }
    }
}

impl fmt::Debug for ArrayRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Display a JSON array as a string.
impl fmt::Display for ArrayRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serialize_in_json(self, f)
    }
}

impl PartialEq for ArrayRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().eq(other.iter())
    }
}

impl Eq for ArrayRef<'_> {}

impl PartialOrd for ArrayRef<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArrayRef<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Array with n elements > array with n - 1 elements
        match self.len().cmp(&other.len()) {
            std::cmp::Ordering::Equal => self.iter().cmp(other.iter()),
            ord => ord,
        }
    }
}

impl Hash for ArrayRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in self.iter() {
            v.hash(state);
        }
    }
}

/// A reference to a JSON object.
#[derive(Clone, Copy)]
pub struct ObjectRef<'a> {
    // # layout
    //      v-v------ \-----\
    // | elements | [kptr, vptr] x len | len | size |
    // |          |     4 x 2 x len    |  4  |  4   |
    // |<-------------- data (size) --------------->|^ptr
    //
    // entries are ordered by key and each key is unique.
    data: &'a [u8],
}

impl<'a> ObjectRef<'a> {
    /// Returns the value associated with the given key, or `None` if the key is not present.
    pub fn get(self, key: &str) -> Option<ValueRef<'a>> {
        // do binary search since entries are ordered by key
        let entries = self.entries();
        let idx = entries
            .binary_search_by_key(&key, |&(kentry, _)| {
                ValueRef::from_slice(self.data, kentry)
                    .as_str()
                    .expect("key must be string")
            })
            .ok()?;
        let (_, ventry) = entries[idx];
        Some(ValueRef::from_slice(self.data, ventry))
    }

    /// Returns the number of elements in the object.
    pub fn len(self) -> usize {
        (&self.data[self.data.len() - 8..]).get_u32_ne() as usize
    }

    /// Returns `true` if the object contains no elements.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the object's key-value pairs.
    pub fn iter(self) -> impl ExactSizeIterator<Item = (&'a str, ValueRef<'a>)> {
        self.entries().iter().map(move |&(kentry, ventry)| {
            let k = ValueRef::from_slice(self.data, kentry);
            let v = ValueRef::from_slice(self.data, ventry);
            (k.as_str().expect("key must be string"), v)
        })
    }

    /// Returns an iterator over the object's keys.
    pub fn keys(self) -> impl ExactSizeIterator<Item = &'a str> {
        self.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the object's values.
    pub fn values(self) -> impl ExactSizeIterator<Item = ValueRef<'a>> {
        self.iter().map(|(_, v)| v)
    }

    /// Returns the entire object as a slice.
    pub(crate) fn as_slice(self) -> &'a [u8] {
        self.data
    }

    /// Creates an `ObjectRef` from a slice.
    fn from_slice(data: &'a [u8], end: usize) -> Self {
        let size = (&data[end - 4..end]).get_u32_ne() as usize;
        Self {
            data: &data[end - size..end],
        }
    }

    /// Returns the key-value entries.
    fn entries(self) -> &'a [(Entry, Entry)] {
        let len = self.len();
        let base = self.data.len() - 8 - 8 * len;
        let slice = &self.data[base..base + 8 * len];
        unsafe { std::slice::from_raw_parts(slice.as_ptr() as _, len) }
    }
}

impl fmt::Debug for ObjectRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Display a JSON object as a string.
impl fmt::Display for ObjectRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serialize_in_json(self, f)
    }
}

impl PartialEq for ObjectRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().eq(other.iter())
    }
}

impl Eq for ObjectRef<'_> {}

impl PartialOrd for ObjectRef<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectRef<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Object with n pairs > object with n - 1 pairs
        match self.len().cmp(&other.len()) {
            std::cmp::Ordering::Equal => self.iter().cmp(other.iter()),
            ord => ord,
        }
    }
}

impl Hash for ObjectRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

/// Serialize a value in JSON format.
fn serialize_in_json(value: &impl ::serde::Serialize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use std::io;

    struct WriterFormatter<'a, 'b: 'a> {
        inner: &'a mut fmt::Formatter<'b>,
    }

    impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            // Safety: the serializer below only emits valid utf8 when using
            // the default formatter.
            let s = unsafe { std::str::from_utf8_unchecked(buf) };
            self.inner.write_str(s).map_err(io_error)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn io_error(_: fmt::Error) -> io::Error {
        // Error value does not matter because Display impl just maps it
        // back to fmt::Error.
        io::Error::new(io::ErrorKind::Other, "fmt error")
    }

    let alternate = f.alternate();
    let mut wr = WriterFormatter { inner: f };
    if alternate {
        // {:#}
        value
            .serialize(&mut serde_json::Serializer::pretty(&mut wr))
            .map_err(|_| fmt::Error)
    } else {
        // {}
        value
            .serialize(&mut serde_json::Serializer::new(&mut wr))
            .map_err(|_| fmt::Error)
    }
}

/// A type that can be used to index into a `ValueRef`.
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: ValueRef<'v>) -> Option<ValueRef<'v>>;
}

impl Index for usize {
    fn index_into<'v>(&self, v: ValueRef<'v>) -> Option<ValueRef<'v>> {
        match v {
            ValueRef::Array(a) => a.get(*self),
            _ => None,
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: ValueRef<'v>) -> Option<ValueRef<'v>> {
        match v {
            ValueRef::Object(o) => o.get(self),
            _ => None,
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: ValueRef<'v>) -> Option<ValueRef<'v>> {
        match v {
            ValueRef::Object(o) => o.get(self),
            _ => None,
        }
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: ValueRef<'v>) -> Option<ValueRef<'v>> {
        (**self).index_into(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}
