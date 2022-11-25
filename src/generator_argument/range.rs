use super::RangeParameter;
use num;
use rand::{distributions::uniform::SampleUniform, prelude::*};
use derive_new;

/// Generator of random number with a upper and lower bound.
#[derive(derive_new::new)]
pub struct RandomBoundedNumberRange<T: num::Num + SampleUniform + std::cmp::PartialOrd + Copy> {
    /// Lower bound.
    lower: T,
    /// Upper bound.
    upper: T,
}

impl<T: num::Num + SampleUniform + std::cmp::PartialOrd + Copy> RangeParameter<T>
    for RandomBoundedNumberRange<T>
{
    fn next(&self) -> T {
        let mut rng = thread_rng();
        rng.gen_range(self.lower..self.upper)
    }
}
