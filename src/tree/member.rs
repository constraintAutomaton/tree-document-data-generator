use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
/// Member inside a [Node](`super::node::Node`).
pub struct Member {
    /// Url of the member.
    pub url: String,
    /// Properties of the member
    pub properties: HashMap<Property, Value>,
}

/// A property url.
type Property = String;
