use core::{hash::Hash, iter::Sum, ops::Add};
use std::ops::AddAssign;

use serde::Serialize;

use super::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Cost(Scalar);

impl Cost {
    pub fn new(value: Scalar) -> Self {
        Cost(value)
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn into_inner(self) -> Scalar {
        self.0
    }
}

impl Eq for Cost {}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl Hash for Cost {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl Add<Scalar> for Cost {
    type Output = Cost;

    fn add(self, rhs: Scalar) -> Self::Output {
        Cost(self.0 + rhs)
    }
}

impl AddAssign<Scalar> for Cost {
    fn add_assign(&mut self, rhs: Scalar) {
        self.0 += rhs;
    }
}

impl Sum<f32> for Cost {
    fn sum<I: Iterator<Item = f32>>(iter: I) -> Self {
        Self(f32::sum(iter))
    }
}

impl Sum<Cost> for Cost {
    fn sum<I: Iterator<Item = Cost>>(iter: I) -> Self {
        Self::sum(iter.map(|cost| cost.0))
    }
}
