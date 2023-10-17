//! Serde support for `ValueRef` and `Builder`.

use std::fmt;

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq};

use crate::{ArrayRef, Builder, ObjectRef, Value, ValueRef};

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl Serialize for ValueRef<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Number(n) => n.serialize(serializer),
            Self::String(s) => serializer.serialize_str(s),
            Self::Array(v) => v.serialize(serializer),
            Self::Object(o) => o.serialize(serializer),
        }
    }
}

impl Serialize for ArrayRef<'_> {
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

impl Serialize for ObjectRef<'_> {
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

impl<'de, W: AsMut<Vec<u8>>> DeserializeSeed<'de> for &mut Builder<W> {
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        impl<'de, W: AsMut<Vec<u8>>> Visitor<'de> for &mut Builder<W> {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<(), E> {
                Ok(self.add_bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<(), E> {
                Ok(self.add_i64(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<(), E> {
                Ok(self.add_u64(value))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<(), E> {
                Ok(self.add_f64(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<(), E>
            where
                E: serde::de::Error,
            {
                Ok(self.add_string(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<(), E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                self.deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<(), E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<(), V::Error>
            where
                V: SeqAccess<'de>,
            {
                self.begin_array();
                while visitor.next_element_seed(&mut *self)?.is_some() {}
                Ok(self.finish_array())
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<(), V::Error>
            where
                V: MapAccess<'de>,
            {
                self.begin_object();
                while visitor.next_key_seed(&mut *self)?.is_some() {
                    visitor.next_value_seed(&mut *self)?;
                }
                Ok(self.finish_object())
            }
        }

        deserializer.deserialize_any(self)
    }
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
}
