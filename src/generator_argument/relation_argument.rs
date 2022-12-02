use super::RangeParameter;
use crate::tree::relation::Relation;
use crate::tree::value::ValueType;
use std::vec::Vec;

/// The type of TREE relation generator.
pub enum RelationGeneratorArg<T> {
    /// Generate the exact relations dictated by the user.
    Direct(Vec<Vec<Relation>>),
    /// Generate the [`Relation`] based on a template
    /// and make the [`Relation`] value vary following a [`RangeParameter`].
    ValueVariation(TemplateRangeVariationRelation<T>),
}

pub struct TemplateRangeVariationRelation<T> {
    /// template of the [`Relation`].
    pub template: Relation,
    /// range of the selected property.
    pub range: Box<dyn RangeParameter<T>>,
    /// distribution of  the [`Relation`] inside the [Node](`crate::tree:node::Node`).
    pub distribution_of_relation: DistributionOfRelation,
    /// Value type of the [`Relation`]
    pub value_type: ValueType,
}

/// distribution of the [`Relation`] inside the [Node](`crate::tree:node::Node`).
pub enum DistributionOfRelation {
    /// directly set the number of relation by node.
    Direct(Vec<usize>),
    /// Set a random number of relation using a [`RangeParameter`] with a number of node.
    Random(Box<dyn RangeParameter<usize>>, usize),
}
