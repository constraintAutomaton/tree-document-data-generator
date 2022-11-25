use super::relation::Relation;
use std::collections::HashMap;
use std::vec::Vec;
use derive_new;
use derive_getters;

#[derive(derive_new::new, Clone, derive_getters::Getters)]
pub struct Node {
    relation: Vec<Relation>,
    node_url: String,
    members: HashMap<String, String>,
}
