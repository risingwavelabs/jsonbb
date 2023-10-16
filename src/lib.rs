//! A JSONB-like binary format for JSON.
//!
//!
//! # Format
//!
//! ptr: type (3 bits) | offset (29 bits)
//!
//! - Null
//!     - ptr: 0x0
//!     - payload: []
//! - Bool
//!     - ptr: 0x1 (false) / 0x2 (true)
//!     - payload: []
//! - Number
//!     - ptr: 0x3 | offset
//!     - payload: kind (u8) + u64 / i64 / f64
//!                ^ptr
//! - String
//!     - ptr: 0x4 | offset
//!     - payload: len (u32) + bytes
//!                ^ptr
//! - Array
//!     - ptr: 0x5 | offset
//!     - payload: [elem] x n + n (u32) + start_offset (u32) + [eptr] x n
//!                ^start       ^ptr
//! - Object
//!     - ptr: 0x6 | offset
//!     - payload: [key, value] x n + n (u32) + start_offset (u32) + [kptr, vptr] x n
//!                ^start             ^ptr
//!                start_offset = ptr - start

use std::fmt;

mod builder;
mod entry;
mod serde;
mod value;
mod value_ref;

pub use self::builder::*;
use self::entry::*;
pub use self::value::*;
pub use self::value_ref::*;
