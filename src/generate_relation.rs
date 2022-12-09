use super::convert_number_to_sparql_string;
use super::generator_argument::relation_argument::{
    DistributionOfRelation, RelationGeneratorArg, RelationTemplate, TemplateRangeVariationRelation,
};
use super::generator_argument::RangeParameter;
use super::tree::relation::Relation;
use super::tree::value::{Value, ValueType};
use num;
use std::fmt::Debug;
use std::vec::Vec;
use uuid;

/// Generate the relations from the generator argument.
pub(super) fn generate_relations<T: num::ToPrimitive + Debug>(
    relation_args: &RelationGeneratorArg<T>,
    base_url: &String,
) -> Result<Vec<Vec<Relation>>, &'static str> {
    match relation_args {
        RelationGeneratorArg::Direct(r) => Ok(r.clone()),

        RelationGeneratorArg::ValueVariation(template) => {
            generate_relations_from_template(template, base_url)
        }
    }
}

/// Generate the relations from a template.
fn generate_relations_from_template<T: num::ToPrimitive + Debug>(
    template: &TemplateRangeVariationRelation<T>,
    base_url: &String,
) -> Result<Vec<Vec<Relation>>, &'static str> {
    let mut relations: Vec<Vec<Relation>> = Vec::new();
    match &template.distribution_of_relation {
        DistributionOfRelation::Direct(dist) => {
            for n in dist.iter() {
                match generate_n_relation_from_a_template(
                    *n,
                    &template.template,
                    &base_url,
                    template.value_type,
                    template.range.as_ref(),
                ) {
                    Ok(v) => relations.push(v),
                    Err(e) => return Err(e),
                };
            }
            Ok(relations)
        }

        DistributionOfRelation::Random(range_fn, n) => {
            for _ in 0..*n {
                let n_relation: usize = range_fn.next();
                match generate_n_relation_from_a_template(
                    n_relation,
                    &template.template,
                    &base_url,
                    template.value_type,
                    template.range.as_ref(),
                ) {
                    Ok(v) => relations.push(v),
                    Err(e) => return Err(e),
                };
            }
            Ok(relations)
        }
    }
}

/// Helper function to avoid repetition to generate n relation from a template
fn generate_n_relation_from_a_template<T: num::ToPrimitive + Debug>(
    n: usize,
    template_relation: &RelationTemplate,
    base_url: &String,
    value_type: ValueType,
    range_value_fn: &dyn RangeParameter<T>,
) -> Result<Vec<Relation>, &'static str> {
    let mut current_relation: Vec<Relation> = Vec::new();
    for _ in 0..n {
        match generate_a_relation_from_template(
            template_relation,
            base_url,
            value_type,
            range_value_fn,
        ) {
            Ok(v) => current_relation.push(v),
            Err(e) => return Err(e),
        }
    }
    Ok(current_relation)
}

/// Generate the single relation from the template and the [range generator](`RangeParameter`)
fn generate_a_relation_from_template<T: num::ToPrimitive + Debug>(
    template_relation: &RelationTemplate,
    base_url: &String,
    value_type: ValueType,
    range_value_fn: &dyn RangeParameter<T>,
) -> Result<Relation, &'static str> {
    let value = range_value_fn.next();
    let relation_value = match convert_number_to_sparql_string(value, value_type) {
        Ok(v) => Value {
            value: v,
            value_type: value_type,
        },
        Err(e) => return Err(e),
    };
    let node_url = format!(
        "{base_url}/{id}",
        base_url = base_url,
        id = uuid::Uuid::new_v4()
    );

    Ok(Relation::new(
        None,
        Some(template_relation.path.clone()),
        Some(relation_value),
        node_url,
        Some(template_relation.relation_type.clone()),
    ))
}
#[cfg(test)]
mod tests_generate_a_relation_from_template {
    use crate::generator_argument::relation_argument::RelationTemplate;
    use crate::tree::relation_operator::RelationOperator;
    use crate::tree::shacl_path::ShaclPath;
    use crate::tree::value::ValueType;
    use lazy_static::lazy_static;
    use std::collections::HashSet;

    use super::generate_a_relation_from_template;
    use super::MockRangeGenerator;

    lazy_static! {
        static ref A_PATH: ShaclPath = String::from("ex:path");
        static ref A_RELATION_TYPE: RelationOperator = RelationOperator::EqualThanRelation;
        static ref A_TEMPLATE_RELATION: RelationTemplate = RelationTemplate {
            path: A_PATH.clone(),
            relation_type: A_RELATION_TYPE.clone()
        };
        static ref A_BASE_URL: String = String::from("https://example.com");
    }
    #[test]
    fn should_return_the_right_relation_given_a_valid_value_type() {
        let value_type = ValueType::Int;
        let relation = generate_a_relation_from_template(
            &A_TEMPLATE_RELATION,
            &A_BASE_URL,
            value_type,
            &MockRangeGenerator { val: 8 },
        )
        .unwrap();

        assert_eq!(*relation.remaning_items(), None);
        assert_eq!(*relation.path(), Some(A_PATH.clone()));
        assert_eq!(*relation.relation_type(), Some(A_RELATION_TYPE.clone()));
    }

    #[test]
    fn should_return_an_error_when_the_value_type_is_incompatible_with_the_value_type() {
        let value_type = ValueType::Boolean;
        generate_a_relation_from_template(
            &A_TEMPLATE_RELATION,
            &A_BASE_URL,
            value_type,
            &MockRangeGenerator { val: 8 },
        )
        .expect_err(
            "should return an error when the type is not compatible with the generator value",
        );
    }

    #[test]
    fn should_not_return_the_same_url() {
        let value_type = ValueType::Int;
        let mut usedUrl: HashSet<String> = HashSet::new();

        for i in 0..100 {
            let relation = generate_a_relation_from_template(
                &A_TEMPLATE_RELATION,
                &A_BASE_URL,
                value_type,
                &MockRangeGenerator { val: i },
            )
            .unwrap();
            assert!(usedUrl.get(relation.node()).is_none());
            usedUrl.insert(relation.node().clone());
        }
    }
}

#[cfg(test)]
pub struct MockRangeGenerator<T> {
    pub val: T,
}

#[cfg(test)]
impl<T: Clone> RangeParameter<T> for MockRangeGenerator<T> {
    fn next(&self) -> T {
        self.val.clone()
    }
}
