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

//! Serde support for `ValueRef` and `Builder`.

use std::fmt::{self, Display};

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{self, Impossible, SerializeMap, SerializeSeq};

use crate::{ArrayRef, Builder, NumberRef, ObjectRef, Value, ValueRef};

/// Convert a value that `impl Serialize` into `jsonbb::Value`.
pub fn to_value<T: ser::Serialize>(value: T) -> Result<Value, fmt::Error> {
    let mut builder = Builder::<Vec<u8>>::new();
    value.serialize(&mut builder)?;
    Ok(builder.finish())
}

impl ser::Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl ser::Serialize for ValueRef<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Number(n) => n.serialize(serializer),
            Self::String(s) => serializer.serialize_str(s.as_str()),
            Self::Array(v) => v.serialize(serializer),
            Self::Object(o) => o.serialize(serializer),
        }
    }
}

impl ser::Serialize for NumberRef<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.to_number().serialize(serializer)
    }
}

impl ser::Serialize for ArrayRef<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for v in self.iter() {
            seq.serialize_element(&v)?;
        }
        seq.end()
    }
}

impl ser::Serialize for ObjectRef<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, &v)?;
        }
        map.end()
    }
}

struct BuilderVisitor<'a, W>(&'a mut Builder<W>);

impl<'de, W: AsMut<Vec<u8>>> Visitor<'de> for BuilderVisitor<'_, W> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid JSON value")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<(), E> {
        self.0.add_bool(value);
        Ok(())
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<(), E> {
        self.0.add_i64(value);
        Ok(())
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<(), E> {
        self.0.add_u64(value);
        Ok(())
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<(), E> {
        self.0.add_f64(value);
        Ok(())
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.0.add_string(value);
        Ok(())
    }

    #[inline]
    fn visit_none<E>(self) -> Result<(), E> {
        self.0.add_null();
        Ok(())
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.0.deserialize(deserializer)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<(), E> {
        self.0.add_null();
        Ok(())
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<(), V::Error>
    where
        V: SeqAccess<'de>,
    {
        self.0.begin_array();
        while visitor.next_element_seed(&mut *self.0)?.is_some() {}
        self.0.end_array();
        Ok(())
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<(), V::Error>
    where
        V: MapAccess<'de>,
    {
        if let Some(first_key) = visitor.next_key::<&str>()? {
            #[cfg(feature = "arbitrary_precision")]
            if first_key == "$serde_json::private::Number" {
                let v = visitor.next_value::<&str>()?;
                self.0.add_number_string(v);
                return Ok(());
            }

            self.0.begin_object();

            // First key-value pair.
            self.0.add_string(first_key);
            visitor.next_value_seed(&mut *self.0)?;

            // Subsequent key-value pairs.
            while visitor.next_key_seed(&mut *self.0)?.is_some() {
                visitor.next_value_seed(&mut *self.0)?;
            }
        } else {
            self.0.begin_object();
            // Empty object.
        }
        self.0.end_object();
        Ok(())
    }
}

impl<'de, W: AsMut<Vec<u8>>> DeserializeSeed<'de> for &mut Builder<W> {
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BuilderVisitor(self))
    }
}

/// Jsonbb is a data format.
// https://docs.rs/serde_json/latest/src/serde_json/ser.rs.html#59-454
impl<W: AsMut<Vec<u8>>> ser::Serializer for &mut Builder<W> {
    type Ok = ();

    type Error = std::fmt::Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.add_bool(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.add_i64(v as _);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.add_i64(v as _);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.add_i64(v as _);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.add_i64(v as _);
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.add_i64(v.try_into().map_err(|_| invalid_number())?);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.add_u64(v as _);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.add_u64(v as _);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.add_u64(v as _);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.add_u64(v as _);
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.add_u64(v.try_into().map_err(|_| invalid_number())?);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.add_f64(v as _);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.add_f64(v as _);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.add_string(v.encode_utf8(&mut [0; 4]));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.add_string(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // serialize as byte array
        self.begin_array();
        for byte in v {
            self.add_u64(*byte as _);
        }
        self.end_array();
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.add_null();
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.add_null();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        self.begin_object();
        self.add_string(variant);
        value.serialize(&mut *self)?;
        self.end_object();
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.begin_array();
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.begin_object();
        self.add_string(variant);
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.begin_object();
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.begin_object();
        self.add_string(variant);
        self.serialize_map(Some(len))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.display(value);
        Ok(())
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeTuple for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeTupleStruct for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeTupleVariant for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        self.end_array();
        self.end_object();
        Ok(())
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeMap for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(MapKeySerializer { ser: *self })
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        self.end_object();
        Ok(())
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeStruct for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeMap::end(self)
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeStructVariant for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        self.end_object();
        self.end_object();
        Ok(())
    }
}

impl<W: AsMut<Vec<u8>>> ser::SerializeSeq for &mut Builder<W> {
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        self.end_array();
        Ok(())
    }
}

struct MapKeySerializer<'a, W> {
    ser: &'a mut Builder<W>,
}

impl<'a, W> ser::Serializer for MapKeySerializer<'a, W>
where
    W: AsMut<Vec<u8>>,
{
    type Ok = ();
    type Error = std::fmt::Error;

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(value)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.ser.display(value);
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(key_must_be_a_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.ser.collect_str(value)
    }
}

fn key_must_be_a_string() -> std::fmt::Error {
    // TODO: better error message
    std::fmt::Error
}

fn invalid_number() -> std::fmt::Error {
    // TODO: better error message
    std::fmt::Error
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn test_serde() {
        let json = r#"
        {
            "null": null,
            "false": false,
            "true": true,
            "string": "hello",
            "integer": 43,
            "u64max": 18446744073709551615,
            "i64min": -9223372036854775808,
            "float": 178.5,
            "array": ["hello", "world"]
        }"#;
        let value: Value = json.parse().unwrap();
        assert_eq!(
            format!("{value}"),
            r#"{"array":["hello","world"],"false":false,"float":178.5,"i64min":-9223372036854775808,"integer":43,"null":null,"string":"hello","true":true,"u64max":18446744073709551615}"#
        );
        assert_eq!(
            format!("{value:#}"),
            r#"
{
  "array": [
    "hello",
    "world"
  ],
  "false": false,
  "float": 178.5,
  "i64min": -9223372036854775808,
  "integer": 43,
  "null": null,
  "string": "hello",
  "true": true,
  "u64max": 18446744073709551615
}"#
            .trim()
        );
    }

    #[test]
    fn expect_end_of_input() {
        "1f2".parse::<Value>().unwrap_err();
        "trues".parse::<Value>().unwrap_err();
        "true, false".parse::<Value>().unwrap_err();
    }

    use super::to_value;
    use serde::Serialize;
    use std::{collections::HashMap, fmt::Display, hash::Hash};

    #[test]
    fn test_to_value() {
        /// Test that `value` serializes to `expected`.
        #[track_caller]
        fn test(value: impl Serialize, expected: &str) {
            let actual = to_value(&value).unwrap().to_string();
            assert_eq!(actual, expected);
            assert_eq!(serde_json::to_value(&value).unwrap().to_string(), expected);
        }

        test((), "null");
        test(true, "true");
        test(42i8, "42");
        test(42i16, "42");
        test(42i32, "42");
        test(42i64, "42");
        test(42i128, "42");
        test(42u8, "42");
        test(42u16, "42");
        test(42u32, "42");
        test(42u64, "42");
        test(42u128, "42");
        // FIXME: actual "1.2300000190734863"
        // test(1.23f32, "1.23");
        test(1.23f64, "1.23");

        test('a', "\"a\"");
        test("hello", "\"hello\"");

        test(None as Option<i32>, "null");
        test(Some(42), "42");

        test([1, 2, 3], "[1,2,3]");
        test(vec![1, 2, 3], "[1,2,3]");

        #[derive(Serialize)]
        struct UnitStruct;

        #[derive(Serialize)]
        struct NewtypeStruct(i32);

        #[derive(Serialize)]
        struct TestStruct {
            id: i32,
            name: String,
        }

        #[derive(Serialize)]
        enum TestEnum {
            // UnitVariant
            A,
            // NewTypeVariant
            B(i32),
            // TupleVariant
            C(i32, i32),
            // StructVariant
            D { x: i32, y: i32 },
        }

        test(UnitStruct, "null");
        test(NewtypeStruct(42), "42");

        let s = TestStruct {
            id: 1,
            name: "Alice".to_string(),
        };
        test(s, r#"{"id":1,"name":"Alice"}"#);

        test(TestEnum::A, r#""A""#);
        test(TestEnum::B(42), r#"{"B":42}"#);
        test(TestEnum::C(4, 2), r#"{"C":[4,2]}"#);
        test(TestEnum::D { x: 1, y: 2 }, r#"{"D":{"x":1,"y":2}}"#);

        test(vec![1, 2, 3], "[1,2,3]");
        test((1, "two"), "[1,\"two\"]");

        /// Test that keys are serialized as strings.
        #[track_caller]
        fn test_map_key(key: impl Serialize + Display + Eq + Hash) {
            let expected = format!("{{\"{key}\":\"value\"}}");
            let map = [(key, "value")].into_iter().collect::<HashMap<_, _>>();
            assert_eq!(to_value(&map).unwrap().to_string(), expected);
            assert_eq!(serde_json::to_value(&map).unwrap().to_string(), expected);
        }
        test_map_key("key");
        test_map_key(true);
        test_map_key(42i8);
        test_map_key(42i16);
        test_map_key(42i32);
        test_map_key(42i64);
        // test_map_key(42i128); // not supported by serde_json
        test_map_key(42u8);
        test_map_key(42u16);
        test_map_key(42u32);
        test_map_key(42u64);
        // test_map_key(42u128); // not supported by serde_json
    }
}
