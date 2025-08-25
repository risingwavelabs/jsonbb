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
use rkyv::Place;
use rkyv::{rancor::Fallible, Serialize};
use rkyv::{
    vec::{ArchivedVec, VecResolver},
    Archive,
};

use crate::{entry::Entry, ValueRef};

#[derive(rkyv::Portable, rkyv::bytecheck::CheckBytes)]
#[bytecheck(crate = rkyv::bytecheck)]
#[repr(C)]
/// An archived [`ValueRef`], whose reference can be converted to a [`ValueRef`]
/// without allocating.
///
/// Note that the layout of [`ArchivedValueRef`] is not compatible with [`ArchivedValue`].
///
/// [`ArchivedValue`]: crate::ArchivedValue
pub struct ArchivedValueRef {
    entry: Entry,
    data: ArchivedVec<u8>,
}

impl<'a> Archive for ValueRef<'a> {
    type Archived = ArchivedValueRef;
    // `Entry` is essentially a `[u8; 4]`, which does not need a stateful resolver.
    type Resolver = VecResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        rkyv::munge::munge!(let ArchivedValueRef { entry: Entry {0: entry}, data } = out);
        <[u8; 4] as Archive>::resolve(&self.make_entry(0).0, [(); 4], entry);
        ArchivedVec::resolve_from_len(self.as_slice().len(), resolver, data);
    }
}

impl<'a, S: Fallible + ?Sized + Allocator + Writer> Serialize<S> for ValueRef<'a> {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <[u8; 4] as Serialize<S>>::serialize(&self.make_entry(0).0, serializer)?;
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<'a> From<&'a ArchivedValueRef> for ValueRef<'a> {
    fn from(value: &'a ArchivedValueRef) -> Self {
        ValueRef::from_slice(value.data.as_slice(), value.entry)
    }
}
