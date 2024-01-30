use serde::Serialize;

use crate::    dist_graph::Point ;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Edge(Point, Point);

impl Edge {
    pub fn new(from: Point, to: Point) -> Self {
        Self(from, to)
    }

    pub fn from_tuple((from, to): (Point, Point)) -> Self {
        Self(from, to)
    }
}
