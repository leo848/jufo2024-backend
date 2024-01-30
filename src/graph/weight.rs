use core::iter::Sum;
use std::ops::Add;

use derive_more::Sum;
use serde::Serialize;

use crate::{dist_graph::Scalar, util::HashScalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Sum)]
pub struct Weight(HashScalar);

impl Weight {
    pub fn new(value: Scalar) -> Self {
        Self(HashScalar::new(value))
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> Scalar {
        self.0.into_inner()
    }
}

impl Add<Scalar> for Weight {
    type Output = Self;
    fn add(self, rhs: Scalar) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add for Weight {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0.into_inner()
    }
}

impl Sum<f32> for Weight {
    fn sum<I: Iterator<Item = f32>>(iter: I) -> Self {
        Self::new(iter.sum())
    }
}

impl From<Weight> for f32 {
    fn from(value: Weight) -> Self {
        value.into_inner()
    }
}
