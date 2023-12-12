use core::{hash::Hash, ops::Index};
use std::{ops::Not, slice::SliceIndex};

use itertools::Itertools;
use serde::Serialize;

use crate::graph::{Cost, Edge, Path, Scalar};
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Point(Vec<Scalar>);

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.len());
        self.0
            .iter()
            .map(|comp| comp.to_bits())
            .for_each(|bits| state.write_u32(bits));
    }
}

impl Point {
    pub fn new(values: Vec<Scalar>) -> Self {
        Self(values)
    }

    pub fn dim(&self) -> usize {
        self.0.len()
    }

    pub fn dist_squared(&self, other: &Point) -> Cost {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(comp1, comp2)| (comp1 - comp2) * (comp1 - comp2))
            .sum()
    }

    pub fn dist(&self, other: &Point) -> Cost {
        self.dist_squared(other).sqrt()
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Points(Vec<Point>);

impl Points {
    pub fn try_new(points: Vec<Point>, dim: u8) -> Option<Self> {
        (points.is_empty().not() && points.iter().all(|s| s.dim() == dim as usize))
            .then_some(Points(points))
    }

    pub fn try_new_raw(values: Vec<Vec<Scalar>>, dim: u8) -> Option<Self> {
        Self::try_new(values.into_iter().map(Point::new).collect(), dim)
    }

    pub fn permutations(self) -> impl Iterator<Item = Points> {
        let len = self.len();
        self.0.into_iter().permutations(len).map(Points)
    }

    pub fn into_path(self) -> Path {
        let dim = self[0].dim().try_into().expect("dimension too high");
        Path::try_new(self.0, dim).expect("Already validated")
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = Edge> + '_ {
        self.0
            .iter()
            .cartesian_product(&self.0)
            .filter(|(t, u)| t != u)
            .map(|(p1, p2)| Edge::new(p1.clone(), p2.clone()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Point> + '_ {
        self.0.iter()
    }
}

impl<Idx: SliceIndex<[Point], Output = Point>> Index<Idx> for Points {
    type Output = Point;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}
