use super::relation::Relation;
use derive_getters;
use derive_new;
use std::collections::HashMap;
use std::vec::Vec;

/// A TREE HTTP document with relationships.
#[derive(derive_new::new, Clone, derive_getters::Getters)]
pub struct Node {
    /// All available relationships in the node.
    relation: Vec<Relation>,
    /// Page URL that identifies this node.
    node_url: String,
    /// The members into this node.
    members: HashMap<String, String>,
}
