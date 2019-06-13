use super::{SimpleValue, Value};
use std::ops;

/// Shamelessly taken from `serde_json`, extended to be used with Fauna values.
///
/// Read the
/// [docs](https://docs.rs/serde_json/1.0.39/serde_json/value/trait.Index.html)
pub trait ValueIndex: private::Sealed {
    #[doc(hidden)]
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value>;
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
}

// Prevent users from implementing the ValueIndex trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
}

impl ValueIndex for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Simple(SimpleValue::Array(ref vec)) => vec.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match *v {
            Value::Simple(SimpleValue::Array(ref mut vec)) => vec.get_mut(*self),
            _ => None,
        }
    }
}

impl ValueIndex for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Simple(SimpleValue::Object(ref map)) => map.get(self),
            _ => None,
        }
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match *v {
            Value::Simple(SimpleValue::Object(ref mut map)) => map.get_mut(self),
            _ => None,
        }
    }
}

impl ValueIndex for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
}

impl<'a, T: ?Sized> ValueIndex for &'a T
where
    T: ValueIndex,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
}

impl<I: ValueIndex> ops::Index<I> for Value {
    type Output = Value;

    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::null();
        index.index_into(self).unwrap_or(&NULL)
    }
}
