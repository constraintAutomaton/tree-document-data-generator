use super::shacl_path::ShaclPath;
use super::value::Value;
use derive_getters;
use derive_new;

#[derive(derive_new::new, Clone, derive_getters::Getters)]
/// Represents a relationship between the members across two nodes.
pub struct Relation {
    /// How many members can be reached when following this relation.
    remaning_items: Option<i32>,
    /// A property path, as defined by SHACL, that indicates what resource the tree:value affects.
    path: Option<ShaclPath>,
    /// The contextual value of this node.
    value: Option<Value>,
    /// Link to the TREE node document for this relationship.
    node: String,
}
