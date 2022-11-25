use std::vec::Vec;
use crate::tree::relation::Relation;
use super::RangeParameter;

/// The type of TREE relation generator
pub enum RelationGeneratorArg<T> {
    /// Generate the exact relations dictated by the user.
    Direct(Vec<Relation>),
    /// Generate the [`Relation`] based on a template
    /// and make the [`Relation`] value vary following a [`RangeParameter`] 
    ValueVariation(TemplateRangeVariationRelation<T>)   
}

pub struct TemplateRangeVariationRelation<T>{
    /// template of the [`Relation`]
    pub template: Relation,
    /// range of the selected property
    pub range: Box<dyn RangeParameter<T>>,
    /// number of [`Relation`]
    pub n: u8
}