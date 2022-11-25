pub struct Value {
    pub value: string,
    pub value_type: ValueType
}

pub enum ValueType {
    Integer = "http://www.w3.org/2001/XMLSchema#integer",
    Decimal = "http://www.w3.org/2001/XMLSchema#decimal",
    Float = "http://www.w3.org/2001/XMLSchema#float",
    Double = "http://www.w3.org/2001/XMLSchema#double",
    String = "http://www.w3.org/2001/XMLSchema#string",
    Boolean = "http://www.w3.org/2001/XMLSchema#boolean",
    DateTime = "http://www.w3.org/2001/XMLSchema#dateTime",

    NonPositiveInteger = "http://www.w3.org/2001/XMLSchema#nonPositiveInteger",
    NegativeInteger = "http://www.w3.org/2001/XMLSchema#negativeInteger",
    Long = "http://www.w3.org/2001/XMLSchema#long",
    Int = "http://www.w3.org/2001/XMLSchema#int",
    Short = "http://www.w3.org/2001/XMLSchema#short",
    Byte = "http://www.w3.org/2001/XMLSchema#byte",
    NonNegativeInteger = "http://www.w3.org/2001/XMLSchema#nonNegativeInteger",
    UnsignedLong = "http://www.w3.org/2001/XMLSchema#nunsignedLong",
    UnsignedInt = "http://www.w3.org/2001/XMLSchema#unsignedInt",
    UnsignedShort = "http://www.w3.org/2001/XMLSchema#unsignedShort",
    UnsignedByte = "http://www.w3.org/2001/XMLSchema#unsignedByte",
    PositiveInteger = "http://www.w3.org/2001/XMLSchema#positiveInteger",
}