mod generate_relation;
pub mod generator_argument;
pub mod sparql_converter;
mod tests;
pub mod tree;

use self::generate_relation::generate_relations;
use generator_argument::Args;
use num;
use std::fmt::Debug;
use std::vec::Vec;
use tree::node::Node;

pub fn generate_tree_document<T: num::ToPrimitive + Debug>(args: &Args<T>) -> Vec<Node> {
    let mut tree_document: Vec<Node> = Vec::new();

    let relations = generate_relations(&args.relation, &args.base_url);

    tree_document
}
