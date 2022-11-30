use std::vec::Vec;
use crate::tree::relation::Relation;
use super::RangeParameter;

/// The type of TREE relation generator.
pub enum RelationGeneratorArg<T> {
    /// Generate the exact relations dictated by the user.
    Direct(Vec<Vec<Relation>>),
    /// Generate the [`Relation`] based on a template
    /// and make the [`Relation`] value vary following a [`RangeParameter`]. 
    ValueVariation(TemplateRangeVariationRelation<T>)   
}

pub struct TemplateRangeVariationRelation<T>{
    /// template of the [`Relation`].
    pub template: Relation,
    /// range of the selected property.
    pub range: Box<dyn RangeParameter<T>>,
    /// distribution of  the [`Relation`] inside the [Node](`crate::tree:node::Node`).
    pub distribution_of_relation: DistributionOfRelation,
}

/// distribution of the [`Relation`] inside the [Node](`crate::tree:node::Node`).
pub enum DistributionOfRelation{
    /// directly set the number of relation by node.
    Direct(Vec<u8>),
    /// Set a random number of relation using a [`RangeParameter`] with a number of node.
    Random(Box<dyn RangeParameter<u8>>, u8)
}
