use std::fmt;

#[derive(Clone)]
/// The type of the relationship.
/// https://treecg.github.io/specification/#vocabulary
pub enum RelationOperator {
    /// All elements in the related node have this prefix.
    PrefixRelation,
    /// All elements in the related node have this substring.
    SubstringRelation,
    /// All members of this related node end with this suffix.
    SuffixRelation,
    /// The related Node’s members are greater than the value. For string comparison,
    /// this relation can refer to a comparison configuration.
    GreaterThanRelation,

    /// Similar to GreaterThanRelation.
    GreaterThanOrEqualToRelation,

    /// Similar to GreaterThanRelation.
    LessThanRelation,
    /// Similar to GreaterThanRelation.
    LessThanOrEqualToRelation,
    /// Similar to GreaterThanRelation.
    EqualThanRelation,

    /// A contains b iff no points of b lie in the exterior of a, and at least one point
    /// of the interior of b lies in the interior of a.
    /// reference http://lin-ear-th-inking.blogspot.com/2007/06/subtleties-of-ogc-covers-spatial.html
    GeospatiallyContainsRelation,
}

impl fmt::Display for RelationOperator{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation: &str = {
            match  self {
                RelationOperator::PrefixRelation => "https://w3id.org/tree#PrefixRelation",
                RelationOperator::SubstringRelation => "https://w3id.org/tree#SubstringRelation",
                RelationOperator::SuffixRelation => "https://w3id.org/tree#SuffixRelation",
                RelationOperator::GreaterThanRelation => "https://w3id.org/tree#GreaterThanRelation",
                RelationOperator::GreaterThanOrEqualToRelation => "https://w3id.org/tree#GreaterThanOrEqualToRelation",
                RelationOperator::LessThanRelation => "https://w3id.org/tree#LessThanRelation",
                RelationOperator::LessThanOrEqualToRelation => "https://w3id.org/tree#LessThanOrEqualToRelation",
                RelationOperator::EqualThanRelation => "https://w3id.org/tree#EqualThanRelation",
                RelationOperator::GeospatiallyContainsRelation => "https://w3id.org/tree#GeospatiallyContainsRelation"
            }
        };
        write!(f, "{}", string_representation)
    }
}