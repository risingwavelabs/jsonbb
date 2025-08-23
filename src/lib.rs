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

//! A JSONB-like binary format for JSON.
//!
//! # Usage
//!
//! `jsonbb` provides an API similar to `serde_json` for constructing and querying JSON values.
//!
//! ```
//! // Deserialize a JSON value from a string of JSON text.
//! let value: jsonbb::Value = r#"{"name": ["foo", "bar"]}"#.parse().unwrap();
//!
//! // Serialize a JSON value into JSON text.
//! let json = value.to_string();
//! assert_eq!(json, r#"{"name":["foo","bar"]}"#);
//! ```
//!
//! As a binary format, you can extract byte slices from it or read JSON values from byte slices.
//!
//! ```
//! # let value: jsonbb::Value = r#"{"name": ["foo", "bar"]}"#.parse().unwrap();
//! // Get the underlying byte slice of a JSON value.
//! let bytes = value.as_bytes();
//!
//! // Read a JSON value from a byte slice.
//! let value = jsonbb::ValueRef::from_bytes(bytes);
//! ```
//!
//! You can use the [`get`] API to subscript a JSON and then build a new JSON using the [`Builder`] API.
//!
//! ```
//! # let value: jsonbb::Value = r#"{"name": ["foo", "bar"]}"#.parse().unwrap();
//! // Subscript a JSON value.
//! let name = value.get("name").unwrap();
//! let foo = name.get(0).unwrap();
//! assert_eq!(foo.as_str().unwrap(), "foo");
//!
//! // Build a JSON value.
//! let mut builder = jsonbb::Builder::<Vec<u8>>::new();
//! builder.begin_object();
//! builder.add_string("name");
//! builder.add_value(foo);
//! builder.end_object();
//! let value = builder.finish();
//! assert_eq!(value.to_string(), r#"{"name":"foo"}"#);
//! ```
//!
//! [`get`]: ValueRef::get
//!
//! # Encoding Format
//!
//! `jsonbb` stores JSON values in contiguous memory. By avoiding dynamic memory allocation, it is
//! more cache-friendly and provides efficient **parsing** and **querying** performance.
//!
//! It has the following key features:
//!
//! 1. Memory Continuity: The content of any JSON subtree is stored contiguously, allowing for
//!    efficient copying through `memcpy`. This leads to highly efficient indexing operations.
//!
//! 2. Post-Order Traversal: JSON nodes are stored in post-order traversal sequence. When parsing
//!    JSON strings, output can be sequentially written to the buffer without additional memory
//!    allocation and movement. This results in highly efficient parsing operations.
//!
//! Each JSON node consists of a fixed-size **entry** and a variable-length **payload**.
//! Each entry is 4 bytes, with 3 bits storing the node type and 29 bits storing the offset of
//! the payload.
//!
//! ```text
//! entry: type (3 bits) | offset (29 bits)
//!
//! # Null
//! entry: 0x0
//! payload: []
//!
//! # Bool
//! entry: 0x1 (false) / 0x2 (true)
//! payload: []
//!
//! # Number
//! entry: 0x3 | offset
//! payload: kind (u8) + u64 / i64 / f64
//!          ^ptr
//!
//! # String
//! entry: 0x4 | offset
//! payload: len (u32) + bytes
//!          ^ptr
//!
//! # Array
//! entry: 0x5 | offset
//! payload: [elem] x n + [entry] x n + n (u32) + len (u32)
//!          ^start                                        ^ptr
//!
//! # Object
//! entry: 0x6 | offset
//! payload: [key, value] x n + [kentry, ventry] x n + n (u32) + len (u32)
//!          ^start                                                       ^ptr
//! where:   len = ptr - start
//! ```

mod builder;
mod entry;
mod macros;
mod partial_eq;
mod serde;
mod value;
mod value_ref;

pub use self::builder::*;
use self::entry::*;
pub use self::serde::*;
pub use self::value::*;
pub use self::value_ref::*;

// for `json!` macro
#[doc(hidden)]
pub use serde_json;
