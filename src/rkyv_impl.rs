// Copyright 2025 RisingWave Labs
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

use rkyv::ser::{Allocator, Writer};
use rkyv::{rancor::Fallible, Serialize};
use rkyv::{
    vec::{ArchivedVec, VecResolver},
    Archive,
};
use rkyv::{Deserialize, Place};

use crate::Value;
use crate::{entry::Entry, ValueRef};

#[derive(rkyv::Portable, rkyv::bytecheck::CheckBytes)]
#[bytecheck(crate = rkyv::bytecheck)]
#[repr(C)]
/// An `rkyv`-archived [`Value`] or [`ValueRef`].
///
/// - It can be deserialized back into a [`Value`].
/// - Its reference can be converted to a [`ValueRef`] without allocating.
pub struct ArchivedValue {
    entry: Entry,
    data: ArchivedVec<u8>,
}

const DUMMY_OFFSET: usize = 0;

impl<'a> Archive for ValueRef<'a> {
    type Archived = ArchivedValue;
    // `Entry` is essentially a `[u8; 4]`, which does not need a stateful resolver.
    type Resolver = VecResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        rkyv::munge::munge!(let ArchivedValue { entry: Entry {0: entry}, data } = out);
        <[u8; 4] as Archive>::resolve(&self.make_entry(DUMMY_OFFSET).0, [(); 4], entry);
        ArchivedVec::resolve_from_len(self.as_slice().len(), resolver, data);
    }
}

impl Archive for Value {
    type Archived = ArchivedValue;
    type Resolver = VecResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        self.as_ref().resolve(resolver, out)
    }
}

impl<'a, S: Fallible + ?Sized + Allocator + Writer> Serialize<S> for ValueRef<'a> {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <[u8; 4] as Serialize<S>>::serialize(&self.make_entry(DUMMY_OFFSET).0, serializer)?;
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<S: Fallible + ?Sized + Allocator + Writer> Serialize<S> for Value {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

impl<D: Fallible> Deserialize<Value, D> for ArchivedValue
where
    <D as Fallible>::Error: rkyv::rancor::Source,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<Value, <D as Fallible>::Error> {
        let entry: [u8; 4] = <[u8; 4]>::deserialize(&self.entry.0, deserializer)?;
        let data: Vec<u8> = <ArchivedVec<u8>>::deserialize(&self.data, deserializer)?;

        let mut buffer = data;
        buffer.extend_from_slice(&entry);
        Ok(Value::from_bytes(buffer))
    }
}

impl<'a> From<&'a ArchivedValue> for ValueRef<'a> {
    fn from(value: &'a ArchivedValue) -> Self {
        ValueRef::from_slice(value.data.as_slice(), value.entry)
    }
}

#[cfg(test)]
mod tests {
    use rkyv::rancor::Error;

    use super::*;

    fn roundtrip_ref(valuer: ValueRef<'_>) {
        let serialized = rkyv::to_bytes::<Error>(&valuer).unwrap();
        let bytes = serialized.to_vec();

        {
            // `Value` and `ValueRef` should serialize to the same bytes.
            let owned_serialized = rkyv::to_bytes::<Error>(&valuer.to_owned()).unwrap();
            let owned_bytes = owned_serialized.to_vec();
            assert_eq!(owned_bytes, bytes);
        }

        // Test that we can directly access `ArchivedValue` from the bytes, and it can be converted
        // back to `ValueRef`.
        let accessed: &ArchivedValue = rkyv::access::<_, Error>(&bytes).unwrap();
        let accessed_valuer = ValueRef::from(accessed);
        assert_eq!(accessed_valuer, valuer);

        // Test that we can also deserialize `Value` from the bytes.
        let deserialized_value: Value = rkyv::from_bytes::<_, Error>(&bytes).unwrap();
        assert_eq!(deserialized_value, valuer.to_owned());

        // Test that the reference of the deserialized `Value` is the same as the original.
        let deserialized_valuer = deserialized_value.as_ref();
        assert_eq!(deserialized_valuer, valuer);

        // Test that the reference of the deserialized `Value` is the same as the one we get from
        // `ArchivedValue`.
        assert_eq!(accessed_valuer, deserialized_valuer);
    }

    fn roundtrip(value: Value) {
        roundtrip_ref(value.as_ref());
    }

    #[test]
    fn test_rkyv() {
        // Simple
        roundtrip(Value::null());
        roundtrip(Value::from(true));
        roundtrip(Value::from(false));
        roundtrip(Value::from(1));
        roundtrip(Value::from(1.0));
        roundtrip(Value::from("hello"));

        // Composite
        roundtrip(Value::from_text(br#"{"a": 1, "b": 2}"#).unwrap());
        roundtrip(Value::from_text(br#"[1, 2, 3]"#).unwrap());
        roundtrip(Value::from_text(br#"{"a": 1, "b": {"c": [4, 5, 6] }}"#).unwrap());

        // Ref
        let value = Value::from_text(br#"{"a": 1, "b": {"c": [5, "6", {"d": 7}] }}"#).unwrap();
        roundtrip_ref(value.as_ref());
        roundtrip_ref(value.pointer("/b").unwrap());
        roundtrip_ref(value.pointer("/b/c").unwrap());
        roundtrip_ref(value.pointer("/b/c/0").unwrap());
        roundtrip_ref(value.pointer("/b/c/1").unwrap());
        roundtrip_ref(value.pointer("/b/c/2").unwrap());
        roundtrip_ref(value.pointer("/b/c/2/d").unwrap());
    }
}
