use core::{hash::Hash, iter::Sum, ops::Index};
use std::slice::SliceIndex;

use bimap::BiMap;
use itertools::Itertools;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Cost(f32);

impl Cost {
    pub fn new(value: f32) -> Self {
        Cost(value)
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}

impl Eq for Cost {}

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

type Scalar = f32;

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
        (points.len() >= 1 && points.iter().all(|s| s.dim() == dim as usize))
            .then(|| Points(points))
    }

    pub fn try_new_raw(values: Vec<Vec<Scalar>>, dim: u8) -> Option<Self> {
        Self::try_new(values.into_iter().map(Point::new).collect(), dim)
    }

    pub fn permutations(self) -> impl Iterator<Item = Points> {
        let len = self.len();
        self.0.into_iter().permutations(len).map(Points)
    }

    pub fn as_path(self) -> Path {
        Path(self.0)
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = Edge> + '_ {
        self.0
            .iter()
            .cartesian_product(&self.0)
            .filter(|(t, u)| t != u)
            .map(|(p1, p2)| Edge(p1.clone(), p2.clone()))
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Edge(Point, Point);

impl Edge {
    pub fn new(from: Point, to: Point) -> Self {
        Self(from, to)
    }

    pub fn from_tuple((from, to): (Point, Point)) -> Self {
        Self(from, to)
    }

    pub fn dist_squared(&self) -> Cost {
        Point::dist_squared(&self.0, &self.1)
    }

    pub fn dist(&self) -> Cost {
        self.dist_squared().sqrt()
    }

    pub fn from(&self) -> &Point {
        &self.0
    }

    pub fn to(&self) -> &Point {
        &self.1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Edges(Vec<Edge>);

impl Edges {
    pub fn from_bimap(map: BiMap<Point, Point>) -> Self {
        map.into_iter().map(|(from, to)| Edge(from, to)).collect()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Edge> {
        self.0.into_iter()
    }
}

impl FromIterator<Edge> for Edges {
    fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
        Edges(iter.into_iter().collect())
    }
}
