use super::RangeParameter;
use rand::{distributions::uniform::SampleUniform, prelude::*};
use std::cmp::PartialOrd;

/// Generator of random number with a upper and lower bound.
pub struct RandomBoundedNumberRange<T: SampleUniform + PartialOrd + Copy> {
    /// Lower bound.
    lower: T,
    /// Upper bound.
    upper: T,
}

impl<T: SampleUniform + PartialOrd + Copy>  RandomBoundedNumberRange<T>{
    pub fn new(lower:T, upper:T)->Self{
        if lower>upper{
            panic!(" \"lower\" variable should be lower or equal to \"upper\" variable");
        }
        RandomBoundedNumberRange { lower: lower, upper: upper }
    }
}

impl<T: SampleUniform + PartialOrd + Copy> RangeParameter<T>
    for RandomBoundedNumberRange<T>
{
    fn next(&self) -> T {
        let mut rng = thread_rng();
        rng.gen_range(self.lower..self.upper)
    }
}
