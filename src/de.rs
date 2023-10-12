//! Deserialization to `Builder`.

use std::fmt;

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::{Builder, Id};

impl<'de> DeserializeSeed<'de> for &mut Builder {
    type Value = Id;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        impl<'de> Visitor<'de> for &mut Builder {
            type Value = Id;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Id, E> {
                Ok(self.add_bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Id, E> {
                Ok(self.add_i64(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Id, E> {
                Ok(self.add_i64(value as i64))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Id, E> {
                Ok(self.add_f64(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Id, E>
            where
                E: serde::de::Error,
            {
                Ok(self.add_string(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Id, E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Id, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                self.deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Id, E> {
                Ok(self.add_null())
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Id, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut ids = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some(elem) = visitor.next_element_seed(&mut *self)? {
                    ids.push(elem);
                }

                Ok(self.add_array(&ids))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Id, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut kvs = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some(key) = visitor.next_key_seed(&mut *self)? {
                    let value = visitor.next_value_seed(&mut *self)?;
                    kvs.push((key, value));
                }

                Ok(self.add_object_ids(&kvs))
            }
        }

        deserializer.deserialize_any(self)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::DeserializeSeed;

    use crate::Builder;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "null": null,
            "false": false,
            "true": true,
            "string": "hello",
            "integer": 43,
            "float": 178.5,
            "array": ["hello", "world"]
        }"#;
        let mut builder = Builder::new();
        let mut deserializer = serde_json::Deserializer::from_str(json);
        let id = builder.deserialize(&mut deserializer).unwrap();
        let value = builder.finish(id);
        assert_eq!(
            format!("{value:#?}"),
            r#"{
    "null": null,
    "false": false,
    "true": true,
    "string": "hello",
    "integer": 43,
    "float": 178.5,
    "array": [
        "hello",
        "world",
    ],
}"#
        );
    }
}
