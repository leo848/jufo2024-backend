pub use edge::Edge;
use itertools::Itertools;
pub use matrix::Matrix;
pub use path::Path;
pub use weight::Weight;

use crate::{
    dist_graph::{Point, Scalar},
    typed::Metric,
};

mod edge;
mod matrix;
mod path;
mod weight;

#[derive(Debug, Clone)]
pub struct Graph {
    pub matrix: Matrix,
}

impl Graph {
    pub fn from_matrix(mat: Matrix) -> Self {
        Graph { matrix: mat }
    }

    pub fn from_values(values: Vec<Vec<Scalar>>) -> Option<Self> {
        Matrix::new(values).map(Self::from_matrix)
    }

    pub fn from_points(points: Vec<Point>, norm: Metric) -> Self {
        let costs = points
            .iter()
            .map(|point1| {
                points
                    .iter()
                    .map(|point2| Point::dist(point1, point2, norm).into_inner())
                    .collect_vec()
            })
            .collect_vec();
        Self::from_values(costs).expect("All vectors should have same length")
    }

    pub fn size(&self) -> usize {
        self.matrix.dim()
    }

    pub fn weight(&self, index1: usize, index2: usize) -> Weight {
        Weight::new(self.matrix[(index1, index2)])
    }
}
