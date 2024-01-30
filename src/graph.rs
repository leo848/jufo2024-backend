pub use edge::Edge;
use itertools::Itertools;
pub use matrix::Matrix;
pub use path::Path;
pub use weight::Weight;

use crate::{
    dist_graph::{Point, Scalar},
    typed::Norm,
};

mod edge;
mod matrix;
mod path;
mod weight;

#[derive(Debug, Clone)]
pub struct Graph {
    mat: Matrix,
}

impl Graph {
    pub fn from_matrix(mat: Matrix) -> Self {
        Graph { mat }
    }

    pub fn from_values(values: Vec<Vec<Scalar>>) -> Option<Self> {
        Matrix::new(values).map(Self::from_matrix)
    }

    pub fn from_points(points: Vec<Point>, norm: Norm) -> Self {
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
        self.mat.dim()
    }

    pub fn weight(&self, index1: usize, index2: usize) -> Weight {
        Weight::new(self.mat[(index1, index2)])
    }

    pub fn path_weight(&self, path: &Path) -> Weight {
        path.iter()
            .tuple_windows()
            .map(|(ni1, ni2)| self.weight(ni1, ni2))
            .sum()
    }
}
