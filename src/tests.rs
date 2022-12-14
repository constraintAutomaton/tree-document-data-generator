#[cfg(test)]
mod tests {
    mod tests_generate_relations {
        use crate::generate_relation::generate_relations;
        use crate::generator_argument::range::RandomBoundedNumberRange;
        use crate::generator_argument::relation_argument::{
            DistributionOfRelation, RelationGeneratorArg, RelationTemplate,
            TemplateRangeVariationRelation,
        };
        use crate::tree::relation::Relation;
        use crate::tree::relation_operator::RelationOperator;
        use crate::tree::value::ValueType;
        use lazy_static;

        lazy_static::lazy_static! {
            static ref A_BASE_URL: String = String::from("https://example.com");
        }
        #[test]
        fn given_a_direct_relation_argument_should_return_the_exact_relations() {
            let relations: Vec<Vec<Relation>> = vec![
                vec![
                    Relation::new(None, None, None, String::from("http://example.com/1"), None),
                    Relation::new(None, None, None, String::from("http://example.com/2"), None),
                    Relation::new(None, None, None, String::from("http://example.com/3"), None),
                ],
                vec![
                    Relation::new(None, None, None, String::from("http://example.com/4"), None),
                    Relation::new(None, None, None, String::from("http://example.com/5"), None),
                ],
            ];
            let arg: RelationGeneratorArg<usize> = RelationGeneratorArg::Direct(relations.clone());

            let resp = generate_relations(&arg, &A_BASE_URL).unwrap();

            assert_eq!(resp, relations);
        }

        #[test]
        fn given_a_value_variation_argument_and_a_direct_distribution_should_return_valid_relations(
        ) {
            let template = RelationTemplate {
                path: String::from("ex:g"),
                relation_type: RelationOperator::LessThanRelation,
            };
            let value_type = ValueType::Float;
            let direct_distribution_relation = vec![1, 2, 3];
            let relation_distribution =
                DistributionOfRelation::Direct(direct_distribution_relation.clone());
            let upper_bound = 10.22;
            let lower_bound = 0.32;
            let template_range_variation = TemplateRangeVariationRelation {
                template: template,
                range: Box::new(RandomBoundedNumberRange::new(lower_bound, upper_bound)),
                distribution_of_relation: relation_distribution,
                value_type: value_type,
            };
            let arg = RelationGeneratorArg::ValueVariation(template_range_variation);

            let resp = generate_relations(&arg, &A_BASE_URL).unwrap();

            assert_eq!(resp.len(), direct_distribution_relation.len());
            for (i, relations_in_node) in resp.iter().enumerate() {
                assert_eq!(relations_in_node.len(), direct_distribution_relation[i]);
                for relation in relations_in_node {
                    let value = relation.value().clone().unwrap();
                    assert_eq!(value.value_type, value_type);
                    let value_as_number: f64 = value
                        .value
                        .parse()
                        .expect("the value should be convertable into a f64");
                    assert!(value_as_number >= lower_bound && value_as_number <= upper_bound);
                }
            }
        }

        #[test]
        fn given_a_value_variation_argument_and_a_random_distribution_should_return_valid_relations(
        ) {
            let template = RelationTemplate {
                path: String::from("ex:g"),
                relation_type: RelationOperator::LessThanRelation,
            };
            let value_type = ValueType::Float;

            let upper_number_relation = 10;
            let lower_number_relation = 5;
            let n_node = 7;
            let relation_distribution = DistributionOfRelation::Random(
                Box::new(RandomBoundedNumberRange::new(
                    lower_number_relation,
                    upper_number_relation,
                )),
                n_node,
            );
            let upper_bound = 10.22;
            let lower_bound = 0.32;
            let template_range_variation = TemplateRangeVariationRelation {
                template: template,
                range: Box::new(RandomBoundedNumberRange::new(lower_bound, upper_bound)),
                distribution_of_relation: relation_distribution,
                value_type: value_type,
            };
            let arg = RelationGeneratorArg::ValueVariation(template_range_variation);

            let resp = generate_relations(&arg, &A_BASE_URL).unwrap();

            assert_eq!(resp.len(), n_node);

            for (i, relations_in_node) in resp.iter().enumerate() {
                assert!(
                    relations_in_node.len() >= lower_number_relation
                        && relations_in_node.len() <= upper_number_relation
                );
                for relation in relations_in_node {
                    let value = relation.value().clone().unwrap();
                    assert_eq!(value.value_type, value_type);
                    let value_as_number: f64 = value
                        .value
                        .parse()
                        .expect("the value should be convertable into a f64");
                    assert!(value_as_number >= lower_bound && value_as_number <= upper_bound);
                }
            }
        }
    }
}
