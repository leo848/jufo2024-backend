use core::{ops::Index, slice::SliceIndex};
use std::ops::Add;

use itertools::Itertools;
use serde::Serialize;

use crate::graph::{Cost, Edge, Edges, Point, Scalar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Path(Vec<Point>);

impl Path {
    pub fn try_new(values: Vec<Point>, dim: u8) -> Option<Self> {
        (values.iter().all(|s| s.dim() == dim as usize)).then_some(Path(values))
    }

    pub fn try_new_raw(values: Vec<Vec<Scalar>>, dim: u8) -> Option<Self> {
        Self::try_new(values.into_iter().map(Point::new).collect(), dim)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn into_inner(self) -> Vec<Point> {
        self.0
    }

    pub fn cost(&self) -> Cost {
        self.0.windows(2).map(|s| s[0].dist(&s[1])).sum()
    }

    pub fn into_edges(self) -> Edges {
        self.0
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(Edge::from_tuple)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, point: Point) {
        self.0.push(point);
    }

    pub fn into_slice(self, range: impl SliceIndex<[Point], Output = [Point]>) -> Path {
        Path(self.0[range].to_vec())
    }

    pub fn slice(&self, range: impl SliceIndex<[Point], Output = [Point]>) -> Path {
        self.clone().into_slice(range)
    }

    pub fn rev(mut self) -> Path {
        self.0.reverse();
        self
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
