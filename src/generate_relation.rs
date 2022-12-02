use super::generator_argument::relation_argument::{
    DistributionOfRelation, RelationGeneratorArg, TemplateRangeVariationRelation,
};
use super::generator_argument::RangeParameter;
use num;
use std::fmt::Debug;
use std::vec::Vec;
use super::tree::relation::Relation;
use super::convert_number_to_sparql_string;
use super::tree::value::{Value, ValueType};
use uuid;

pub(super) fn generate_relations<T: num::ToPrimitive + Debug>(relation_args: &RelationGeneratorArg<T>, base_url: &String) -> Result<Vec<Vec<Relation>>, &'static str> {
    match relation_args {
        RelationGeneratorArg::Direct(r) => Ok(r.clone()),

        RelationGeneratorArg::ValueVariation(template) => {
            generate_relations_from_template(template, base_url)
        }
    }
}

fn generate_relations_from_template<T: num::ToPrimitive + Debug>(
    template: &TemplateRangeVariationRelation<T>,
    base_url: &String,
) -> Result<Vec<Vec<Relation>>, &'static str> {
    let mut relations: Vec<Vec<Relation>> = Vec::new();
    match &template.distribution_of_relation {
        DistributionOfRelation::Direct(dist) => {
            for n in dist.iter() {
                match generate_n_relation_from_template(*n,&template.template, &base_url,template.value_type,&template.range){
                    Ok(v)=> relations.push(v),
                    Err(e)=>return Err(e),
                };
            }
            Ok(relations)
        }

        DistributionOfRelation::Random(range_fn, n) => {
            for _ in 0..*n{
                let n_relation:usize = range_fn.next();
                match generate_n_relation_from_template(n_relation,&template.template, &base_url,template.value_type,&template.range){
                    Ok(v)=> relations.push(v),
                    Err(e)=>return Err(e),
                };
            }
            Ok(relations)
        }
    }
}

fn generate_n_relation_from_template<T:  num::ToPrimitive + Debug >(
    n: usize,
    template_relation: &Relation,
    base_url:& String,
    value_type: ValueType,
    range_value_fn: &Box<dyn RangeParameter<T>>,)-> Result<Vec<Relation>, &'static str>{
    let mut current_relation:Vec<Relation> = Vec::new();
    for _ in 0..n{
        match  generate_a_relation_from_template(template_relation, base_url,value_type,range_value_fn){
            Ok(v)=> current_relation.push(v),
            Err(e)=> return Err(e),
        }
    }
    Ok(current_relation)
}

fn generate_a_relation_from_template<T: num::ToPrimitive + Debug>(
    template_relation: &Relation,
    base_url: &String,
    value_type: ValueType,
    range_value_fn: &Box<dyn RangeParameter<T>>,
) -> Result<Relation, &'static str> {
    let value = range_value_fn.next();
    let relation_value = match convert_number_to_sparql_string(value, value_type) {
        Ok(v)=> Value{
            value: v,
            value_type: value_type
        },
        Err(e)=> return  Err(e)
    };
    let node_url = format!("{base_url}/{id}", base_url=base_url, id=uuid::Uuid::new_v4());

    Ok(Relation::new(None, template_relation.path().clone(), Some(relation_value),node_url))
}
