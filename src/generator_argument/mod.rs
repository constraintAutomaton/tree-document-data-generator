pub mod range;
pub mod relation_argument;

use self::relation_argument::RelationGeneratorArg;
pub trait RangeParameter<T> {
    /// get the next value.
    fn next(&self) -> T;
}
/// Argument necessary to generate a TREE document at the user request.
pub struct Args<T> {
    pub relation: RelationGeneratorArg<T>,
    pub first_node_url: String,
}
