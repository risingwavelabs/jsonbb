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
