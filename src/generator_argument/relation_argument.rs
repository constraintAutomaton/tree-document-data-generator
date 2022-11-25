use std::vec::Vec;
use crate::tree::relation::Relation;
use super::RangeParameter;

pub enum RelationGeneratorArg<T> {
    Direct(Vec<Relation>),
    ValueVariation(TemplateRangeVariationRelation<T>)   
}

pub struct TemplateRangeVariationRelation<T>{
    pub template: Relation,
    pub range: Box<dyn RangeParameter<T>>,
    pub n: u8
}