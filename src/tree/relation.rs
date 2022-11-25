use super::shacl_path::ShaclPath;
use super::value::Value;
use derive_new;
use derive_getters;

#[derive(derive_new::new, Clone, derive_getters::Getters)]
pub struct Relation {
    remaning_items: Option<i32>,
    path: Option<ShaclPath>,
    value: Option<Value>,
    node: String,
}
