use super::shacl_path::ShaclPath;
use super::value::Value;
pub struct Relation{
    pub remaning_items: Option<i32>,
    pub path: Option<ShaclPath>,
    pub value: Option<Value>,
    pub node: String,
}