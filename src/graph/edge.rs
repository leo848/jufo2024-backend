use bimap::BiMap;
use serde::Serialize;

use crate::{
    graph::{Cost, Point},
    typed::Norm,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Edge(Point, Point);

impl Edge {
    pub fn new(from: Point, to: Point) -> Self {
        Self(from, to)
    }

    pub fn from_tuple((from, to): (Point, Point)) -> Self {
        Self(from, to)
    }

    pub fn comparable_dist(&self, norm: Norm) -> Cost {
        Point::comparable_dist(&self.0, &self.1, norm)
    }

    pub fn dist(&self, norm: Norm) -> Cost {
        Point::dist(&self.0, &self.1, norm)
    }

    pub fn from(&self) -> &Point {
        &self.0
    }

    pub fn to(&self) -> &Point {
        &self.1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Edges(Vec<Edge>);

impl Edges {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, edge: Edge) {
        self.0.push(edge);
    }

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
