use core::{ops::Index, slice::SliceIndex};
use std::ops::Add;

use itertools::Itertools;
use serde::Serialize;

use crate::{
    graph::{Cost, Edge, Edges, Point, Scalar},
    typed::Norm,
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

    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn into_inner(self) -> Vec<Point> {
        self.0
    }

    pub fn cost(&self, norm: Norm) -> Cost {
        self.0.windows(2).map(|s| s[0].dist(&s[1], norm)).sum()
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

    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    pub fn get(&self, index: usize) -> Option<&Point> {
        self.0.get(index)
    }

    fn cost_delta_under_swap_next(&self, index: usize, norm: Norm) -> Scalar {
        assert!(index + 1 < self.len());
        let (ll, rr) = (self.get(index.wrapping_sub(1)), self.get(index + 2));
        let (lv, rv) = (self.get(index).unwrap(), self.get(index).unwrap());
        let cmp = |one: &Point, two: Option<&Point>| {
            two.map(|two| (one, two))
                .map(|(one, two)| Point::dist(&one, &two, norm).into_inner())
                .unwrap_or(0.0)
        };
        cmp(&lv, rr) + cmp(&rv, ll) - cmp(&lv, ll) - cmp(&rv, rr)
    }

    pub fn cost_delta_under_swap(&self, index1: usize, index2: usize, norm: Norm) -> Scalar {
        if index1 == index2 {
            return 0.0;
        }
        if index1 + 1 == index2 {
            return self.cost_delta_under_swap_next(index1, norm);
        }
        if index2 + 1 == index1 {
            return self.cost_delta_under_swap_next(index2, norm);
        }
        assert!(index1 < self.len() && index2 < self.len());
        let (lv, rv) = (self.get(index1).unwrap(), self.get(index2).unwrap());
        let get = |idx: usize| self.get(idx);
        let (ll, lr) = (get(index1.wrapping_sub(1)), get(index1.wrapping_add(1)));
        let (rl, rr) = (get(index2.wrapping_sub(1)), get(index2.wrapping_add(1)));
        let cmp = |one: &Point, two: Option<&Point>| {
            two.map(|two| (one, two))
                .map(|(one, two)| Point::dist(one, two, norm).into_inner())
                .unwrap_or(0.0)
        };
        cmp(&rv, ll) + cmp(&rv, lr) + cmp(&lv, rl) + cmp(&lv, rr)
            - cmp(&lv, ll)
            - cmp(&lv, lr)
            - cmp(&rv, rl)
            - cmp(&rv, rr)
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
