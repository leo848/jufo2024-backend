#![allow(clippy::range_plus_one)]
pub fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

use core::{hash::Hash, iter::Sum, ops::Add};
use std::ops::AddAssign;

use serde::Serialize;

use crate::dist_graph::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct HashScalar(Scalar);

impl HashScalar {
    pub fn new(value: Scalar) -> Self {
        HashScalar(value)
    }

    pub fn into_inner(self) -> Scalar {
        self.0
    }
}

impl Eq for HashScalar {}

impl PartialOrd for HashScalar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashScalar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl Hash for HashScalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl Add<Scalar> for HashScalar {
    type Output = HashScalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        HashScalar(self.0 + rhs)
    }
}

impl AddAssign<Scalar> for HashScalar {
    fn add_assign(&mut self, rhs: Scalar) {
        self.0 += rhs;
    }
}

impl Sum<Scalar> for HashScalar {
    fn sum<I: Iterator<Item = Scalar>>(iter: I) -> Self {
        Self(Scalar::sum(iter))
    }
}

impl Sum<HashScalar> for HashScalar {
    fn sum<I: Iterator<Item = HashScalar>>(iter: I) -> Self {
        Self::sum(iter.map(|cost| cost.0))
    }
}
