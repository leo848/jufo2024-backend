use core::iter::Sum;
use std::ops::{Add, Neg};

use derive_more::Sum;
use serde::Serialize;

use crate::{dist_graph::Scalar, util::HashScalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Sum)]
pub struct Cost(HashScalar);

impl Cost {
    pub fn new(value: Scalar) -> Self {
        Self(HashScalar::new(value))
    }

    pub fn into_inner(self) -> Scalar {
        self.0.into_inner()
    }

    pub fn sqrt(self) -> Self {
        Self(HashScalar::new(self.into_inner().sqrt()))
    }
}

impl Add<Scalar> for Cost {
    type Output = Self;
    fn add(self, rhs: Scalar) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add for Cost {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0.into_inner()
    }
}

impl Sum<f32> for Cost {
    fn sum<I: Iterator<Item = f32>>(iter: I) -> Self {
        Self::new(iter.sum())
    }
}

impl From<Cost> for f32 {
    fn from(value: Cost) -> Self {
        value.into_inner()
    }
}

impl Neg for Cost {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Cost(-self.0)
    }
}
