use std::ops::Index;

use crate::dist_graph::Scalar;

/// A quadratic n x n matrix of real numbers.
#[derive(Debug, Clone)]
pub struct Matrix {
    values: Vec<Vec<Scalar>>,
}

impl Matrix {
    pub fn new(values: Vec<Vec<Scalar>>) -> Option<Self> {
        let dim = values.len();
        if values.iter().any(|inner| inner.len() != dim) {
            return None;
        }
        Some(Self { values })
    }

    pub fn dim(&self) -> usize {
        self.values.len()
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Scalar;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[x][y]
    }
}
