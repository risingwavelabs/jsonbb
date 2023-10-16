//! Serde support for `ValueRef` and `Builder`.

use std::fmt;

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq};

use crate::{ArrayRef, Builder, ObjectRef, Ptr, Value, ValueRef};

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

impl<'de> DeserializeSeed<'de> for &mut Builder<'_> {
    type Value = Ptr;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        impl<'de> Visitor<'de> for &mut Builder<'_> {
            type Value = Ptr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Ptr, E> {
                Ok(self.add_bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Ptr, E> {
                Ok(self.add_i64(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Ptr, E> {
                Ok(self.add_u64(value))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Ptr, E> {
                Ok(self.add_f64(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Ptr, E>
            where
                E: serde::de::Error,
            {
                Ok(self.add_string(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Ptr, E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Ptr, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                self.deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Ptr, E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Ptr, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let start = self.len();
                let mut ids = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some(elem) = visitor.next_element_seed(&mut *self)? {
                    ids.push(elem);
                }

                Ok(self.add_array(start, &ids))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Ptr, V::Error>
            where
                V: MapAccess<'de>,
            {
                let start = self.len();
                let mut kvs = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some(key) = visitor.next_key_seed(&mut *self)? {
                    let value = visitor.next_value_seed(&mut *self)?;
                    kvs.push((key, value));
                }

                Ok(self.add_object(start, kvs.into_iter()))
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
            r#"{"null":null,"false":false,"true":true,"string":"hello","integer":43,"u64max":18446744073709551615,"i64min":-9223372036854775808,"float":178.5,"array":["hello","world"]}"#
        );
        assert_eq!(
            format!("{value:#}"),
            r#"
{
  "null": null,
  "false": false,
  "true": true,
  "string": "hello",
  "integer": 43,
  "u64max": 18446744073709551615,
  "i64min": -9223372036854775808,
  "float": 178.5,
  "array": [
    "hello",
    "world"
  ]
}"#
            .trim()
        );
    }
}
