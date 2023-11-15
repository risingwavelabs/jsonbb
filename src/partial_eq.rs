use crate::ValueRef;

use super::Value;
use std::string::String;

fn eq_i64(value: ValueRef<'_>, other: i64) -> bool {
    value.as_i64().map_or(false, |i| i == other)
}

fn eq_u64(value: ValueRef<'_>, other: u64) -> bool {
    value.as_u64().map_or(false, |i| i == other)
}

fn eq_f32(value: ValueRef<'_>, other: f32) -> bool {
    match value {
        ValueRef::Number(n) => n.as_f32().map_or(false, |i| i == other),
        _ => false,
    }
}

fn eq_f64(value: ValueRef<'_>, other: f64) -> bool {
    value.as_f64().map_or(false, |i| i == other)
}

fn eq_bool(value: ValueRef<'_>, other: bool) -> bool {
    value.as_bool().map_or(false, |i| i == other)
}

fn eq_str(value: ValueRef<'_>, other: &str) -> bool {
    value.as_str().map_or(false, |i| i == other)
}

impl PartialEq<str> for ValueRef<'_> {
    fn eq(&self, other: &str) -> bool {
        eq_str(*self, other)
    }
}

impl PartialEq<&str> for ValueRef<'_> {
    fn eq(&self, other: &&str) -> bool {
        eq_str(*self, *other)
    }
}

impl PartialEq<ValueRef<'_>> for str {
    fn eq(&self, other: &ValueRef<'_>) -> bool {
        eq_str(*other, self)
    }
}

impl PartialEq<ValueRef<'_>> for &str {
    fn eq(&self, other: &ValueRef<'_>) -> bool {
        eq_str(*other, *self)
    }
}

impl PartialEq<String> for ValueRef<'_> {
    fn eq(&self, other: &String) -> bool {
        eq_str(*self, other.as_str())
    }
}

impl PartialEq<ValueRef<'_>> for String {
    fn eq(&self, other: &ValueRef<'_>) -> bool {
        eq_str(*other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for ValueRef<'_> {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    $eq(other.as_ref(), *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self.as_ref(), *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f32[f32]
    eq_f64[f64]
    eq_bool[bool]
}
