use std::fmt;

#[derive(Clone)]
pub struct Value {
    pub value: String,
    pub value_type: ValueType,
}

#[derive(Clone)]
pub enum ValueType {
    Integer,
    Decimal,
    Float,
    Double,
    String,
    Boolean,
    DateTime,

    NonPositiveInteger,
    NegativeInteger,
    Long,
    Int,
    Short,
    Byte,
    NonNegativeInteger,
    UnsignedLong,
    UnsignedInt,
    UnsignedShort,
    UnsignedByte,
    PositiveInteger,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_representation: &str = {
            match self {
                ValueType::Integer => "http://www.w3.org/2001/XMLSchema#integer",
                ValueType::Decimal => "http://www.w3.org/2001/XMLSchema#decimal",
                ValueType::Float => "http://www.w3.org/2001/XMLSchema#float",
                ValueType::Double => "http://www.w3.org/2001/XMLSchema#double",
                ValueType::String => "http://www.w3.org/2001/XMLSchema#string",
                ValueType::Boolean => "http://www.w3.org/2001/XMLSchema#boolean",
                ValueType::DateTime => "http://www.w3.org/2001/XMLSchema#dateTime",

                ValueType::NonPositiveInteger => {
                    "http://www.w3.org/2001/XMLSchema#nonPositiveInteger"
                }
                ValueType::NegativeInteger => "http://www.w3.org/2001/XMLSchema#negativeInteger",
                ValueType::Long => "http://www.w3.org/2001/XMLSchema#long",
                ValueType::Int => "http://www.w3.org/2001/XMLSchema#int",
                ValueType::Short => "http://www.w3.org/2001/XMLSchema#short",
                ValueType::Byte => "http://www.w3.org/2001/XMLSchema#byte",
                ValueType::NonNegativeInteger => {
                    "http://www.w3.org/2001/XMLSchema#nonNegativeInteger"
                }
                ValueType::UnsignedLong => "http://www.w3.org/2001/XMLSchema#nunsignedLong",
                ValueType::UnsignedInt => "http://www.w3.org/2001/XMLSchema#unsignedInt",
                ValueType::UnsignedShort => "http://www.w3.org/2001/XMLSchema#unsignedShort",
                ValueType::UnsignedByte => "http://www.w3.org/2001/XMLSchema#unsignedByte",
                ValueType::PositiveInteger => "http://www.w3.org/2001/XMLSchema#positiveInteger",
            }
        };

        write!(f, "Circle of radius {}", string_representation)
    }
}
