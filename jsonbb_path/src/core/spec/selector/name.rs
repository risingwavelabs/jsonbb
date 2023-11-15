//! Name selector for selecting object keys in JSONPath
use jsonbb::ValueRef;

use crate::core::spec::query::Queryable;

/// Select a single JSON object key
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name(pub String);

impl Name {
    /// Get as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{name}'", name = self.0)
    }
}

impl Queryable for Name {
    #[cfg_attr(feature = "trace", tracing::instrument(name = "Query Name", level = "trace", parent = None, ret))]
    fn query<'b>(&self, current: ValueRef<'b>, _root: ValueRef<'b>) -> Vec<ValueRef<'b>> {
        if let Some(obj) = current.as_object() {
            obj.get(&self.0).into_iter().collect()
        } else {
            vec![]
        }
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
