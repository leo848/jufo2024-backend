use itertools::Itertools;
use core::ops::Index;
use core::slice::SliceIndex;
use serde::Serialize;
use crate::graph::{Scalar, Point, Cost, Edge, Edges};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Path(Vec<Point>);

impl Path {
    pub fn try_new(values: Vec<Point>, dim: u8) -> Option<Self> {
        (values.iter().all(|s| s.dim() == dim as usize)).then(|| Path(values))
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

    pub fn to_edges(self) -> Edges {
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
        self.0.push(point)
    }
}

impl<Idx: SliceIndex<[Point], Output = Point>> Index<Idx> for Path {
    type Output = Point;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl AsMut<[Point]> for Path {
    fn as_mut(&mut self) -> &mut [Point] {
        self.0.as_mut()
    }
}

