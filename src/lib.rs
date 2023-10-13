//! A JSONB-like binary format for storing JSON values.

use bytes::Buf;
use std::fmt;

mod array;
mod builder;
mod serde;
mod value;
mod value_ref;

pub use self::array::*;
pub use self::builder::*;
pub use self::value::*;
pub use self::value_ref::*;

/// A key to identify JSON values within a buffer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

impl Id {
    // special values
    const NULL: Self = Id(0x8000_0000);
    const FALSE: Self = Id(0x8000_0001);
    const TRUE: Self = Id(0x8000_0002);
    // otherwise the Id is the offset in the buffer

    const SIZE: usize = std::mem::size_of::<Self>();
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NULL => write!(f, "null"),
            Self::FALSE => write!(f, "false"),
            Self::TRUE => write!(f, "true"),
            Id(i) => write!(f, "#{i}"),
        }
    }
}

const TAG_U64: u8 = 1;
const TAG_I64: u8 = 2;
const TAG_F64: u8 = 3;
const TAG_STRING: u8 = 4;
const TAG_ARRAY: u8 = 5;
const TAG_OBJECT: u8 = 6;
