# jsonbb

[![Crate](https://img.shields.io/crates/v/jsonbb.svg)](https://crates.io/crates/jsonbb)
[![Docs](https://docs.rs/jsonbb/badge.svg)](https://docs.rs/jsonbb)

`jsonbb` is a binary representation of JSON value. It is inspired by [JSONB](https://www.postgresql.org/docs/current/datatype-json.html) in PostgreSQL and optimized for fast parsing.

## Usage

`jsonbb` provides an API similar to `serde_json` for constructing and querying JSON values.

```rust
// Deserialize a JSON value from a string of JSON text.
let value: jsonbb::Value = r#"{"name": ["foo", "bar"]}"#.parse().unwrap();

// Serialize a JSON value into JSON text.
let json = value.to_string();
assert_eq!(json, r#"{"name":["foo","bar"]}"#);
```

As a binary format, you can extract byte slices from it or read JSON values from byte slices.

```rust
// Get the underlying byte slice of a JSON value.
let jsonbb = value.as_bytes();

// Read a JSON value from a byte slice.
let value = jsonbb::ValueRef::from_bytes(jsonbb);
```

You can use common API to query JSON and then build new JSON values using the `Builder` API.

```rust
// Indexing
let name = value.get("name").unwrap();
let foo = name.get(0).unwrap();
assert_eq!(foo.as_str().unwrap(), "foo");

// Build a JSON value.
let mut builder = jsonbb::Builder::<Vec<u8>>::new();
builder.begin_object();
builder.add_string("name");
builder.add_value(foo);
builder.end_object();
let value = builder.finish();
assert_eq!(value.to_string(), r#"{"name":"foo"}"#);
```

## Format

`jsonbb` stores JSON values in contiguous memory. By avoiding dynamic memory allocation, it is more cache-friendly and provides efficient **parsing** and **querying** performance.

It has the following key features:

1. Memory Continuity: The content of any JSON subtree is stored contiguously, allowing for efficient copying through `memcpy`. This leads to highly efficient indexing operations.
2. Post-Order Traversal: JSON nodes are stored in post-order traversal sequence. When parsing JSON strings, output can be sequentially written to the buffer without additional memory allocation and movement. This results in highly efficient parsing operations.

## Performance Comparison

| item[^0]                    | jsonbb    | [jsonb]   | [serde_json]   | [simd_json]    |
| --------------------------- | --------- | --------- | -------------- | -------------- |
| `canada.parse()`            | 4.7394 ms | 12.640 ms | 10.806 ms      | 6.0767 ms [^1] |
| `canada.to_json()`          | 5.7694 ms | 20.420 ms | 5.5702 ms      | 3.0548 ms      |
| `canada.size()`             | 2,117,412 B | 1,892,844 B |            |                |
| `canada["type"]`[^2]        | 39.181 ns[^2.1] | 316.51 ns[^2.2] | 67.202 ns [^2.3] | 27.102 ns [^2.4] |
| `citm_catalog["areaNames"]` | 92.363 ns | 328.70 ns | 2.1190 µs [^3] | 1.9012 µs [^3] |
| `from("1234567890")`        | 26.840 ns | 91.037 ns | 45.130 ns      | 21.513 ns      |
| `a == b`                    | 66.513 ns | 115.89 ns | 39.213 ns      | 41.675 ns      |
| `a < b`                     | 71.793 ns | 120.77 ns | not supported  | not supported  |

[jsonb]: https://docs.rs/jsonb/0.3.0/jsonb/
[serde_json]: https://docs.rs/serde_json/1.0.107/serde_json/
[simd_json]: https://docs.rs/simd-json/0.12.0/simd_json/

[^0]: JSON files for benchmark: [canada](https://github.com/datafuselabs/jsonb/blob/6b3f03effc08e1ca3cad69199e4cb1398e482757/data/canada.json), [citm_catalog](https://github.com/datafuselabs/jsonb/blob/6b3f03effc08e1ca3cad69199e4cb1398e482757/data/citm_catalog.json)

[^1]: Parsed to [`simd_json::OwnedValue`](https://docs.rs/simd-json/0.12.0/simd_json/value/owned/enum.Value.html) for fair.

[^2]: `canada["type"]` returns a string, so the primary overhead of this operation lies in indexing.

[^2.1]: `jsonbb` uses binary search on sorted keys
[^2.2]: `jsonb` uses linear search on unsorted keys
[^2.3]: `serde_json` uses `BTreeMap`
[^2.4]: `simd_json` uses `HashMap`

[^3]: `citm_catalog["areaNames"]` returns an object with 17 key-value string pairs. However, both `serde_json` and `simd_json` exhibit slower performance due to dynamic memory allocation for each string. In contrast, jsonb employs a flat representation, allowing for direct memcpy operations, resulting in better performance.
