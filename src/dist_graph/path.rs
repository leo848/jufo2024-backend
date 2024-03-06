use core::{ops::Index, slice::SliceIndex};
use std::ops::Add;

use itertools::Itertools;
use serde::Serialize;

use crate::{
    dist_graph::{Cost, Edge, Point, Scalar},
    typed::Metric,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Path(Vec<Point>);

impl Path {
    pub fn try_new(values: Vec<Point>, dim: u8) -> Option<Self> {
        (values.iter().all(|s| s.dim() == dim as usize)).then_some(Path(values))
    }

    pub fn try_new_raw(values: Vec<Vec<Scalar>>, dim: u8) -> Option<Self> {
        Self::try_new(values.into_iter().map(Point::new).collect(), dim)
    }

    pub fn into_inner(self) -> Vec<Point> {
        self.0
    }

    pub fn cost(&self, norm: Metric) -> Cost {
        self.0.windows(2).map(|s| s[0].dist(&s[1], norm)).sum()
    }

    pub fn into_edges(self) -> Vec<Edge> {
        self.0
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(Edge::from_tuple)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Add<&Path> for Path {
    type Output = Path;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.0.extend_from_slice(&rhs.0);
        self
    }
}

impl Add for Path {
    type Output = Path;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl<Output: ?Sized, Idx: SliceIndex<[Point], Output = Output>> Index<Idx> for Path {
    type Output = <Idx as SliceIndex<[Point]>>::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl AsMut<[Point]> for Path {
    fn as_mut(&mut self) -> &mut [Point] {
        self.0.as_mut()
    }
}
