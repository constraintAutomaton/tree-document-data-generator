use super::relation::Relation;
use derive_getters;
use derive_new;
use std::vec::Vec;
use super::member::Member;

/// A TREE HTTP document with relationships.
#[derive(derive_new::new, Clone, derive_getters::Getters)]
pub struct Node {
    /// All available relationships in the node.
    relation: Vec<Relation>,
    /// Page URL that identifies this node.
    node_url: String,
    /// The [members](`Member`) into this node.
    members: Vec<Member>,
}
