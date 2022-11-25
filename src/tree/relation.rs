use super::schal_path::SchaclPath;
use super::value::Value;
pub struct Relation{
    pub remaning_iterms: Option<i32>,
    pub path: Option<SchaclPath>,
    pub value: Option<Value>,
    pub node: String,
}