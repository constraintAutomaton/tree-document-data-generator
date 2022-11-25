pub mod relation_argument;
pub mod random_bounded_range;
pub trait RangeParameter<T> {
    fn next(&self) -> T;
}
