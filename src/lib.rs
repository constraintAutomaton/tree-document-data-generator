mod generate_relation;
pub mod generator_argument;
pub mod tree;

use self::generate_relation::generate_relations;
use chrono;
use generator_argument::Args;
use num;
use std::fmt::Debug;
use std::vec::Vec;
use tree::node::Node;
use tree::value::ValueType;

pub fn generate_tree_document<T: num::ToPrimitive + Debug>(args: &Args<T>) -> Vec<Node> {
    let mut tree_document: Vec<Node> = Vec::new();

    let relations = generate_relations(&args.relation, &args.base_url);

    tree_document
}
/// convert a number to [SPARQL](https://www.w3.org/TR/sparql11-query/#operandDataTypes) number compatible operand
pub fn convert_number_to_sparql_string<T: num::ToPrimitive + Debug>(
    number_value: T,
    value_type: ValueType,
) -> Result<String, &'static str> {
    match value_type {
        ValueType::Integer
        | ValueType::Decimal
        | ValueType::Float
        | ValueType::Double
        | ValueType::NonPositiveInteger
        | ValueType::NegativeInteger
        | ValueType::Long
        | ValueType::Int
        | ValueType::Short
        | ValueType::NonNegativeInteger
        | ValueType::UnsignedLong
        | ValueType::UnsignedInt
        | ValueType::UnsignedShort
        | ValueType::PositiveInteger => Ok(format!("{:?}", number_value)),

        ValueType::Boolean => {
            if number_value.to_u8() == Some(1u8) {
                Ok(String::from("true"))
            } else if number_value.to_u8() == Some(0u8) {
                Ok(String::from("false"))
            } else {
                Err("a boolean number should be between 0 and 1")
            }
        }

        ValueType::DateTime => {
            let unix_time = match number_value.to_i64() {
                Some(v) => v,
                None => return Err("for a Datetime the number value should be castable to a i64 "),
            };
            let date = match chrono::NaiveDateTime::from_timestamp_opt(unix_time, 0) {
                Some(v) => v,
                None => return Err("should be able to cast the number value to a date time"),
            };
            Ok(date.format("%Y-%m-%dT%H:%M:%SZ").to_string())
        }
        _ => Err("the type cannot be created from a number"),
    }
}
