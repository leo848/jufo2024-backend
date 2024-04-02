use itertools::Itertools;
use std::{fmt::Debug, ops::Index};

use crate::dist_graph::Scalar;

/// A quadratic n x n matrix of real numbers.
#[derive(Clone)]
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

    pub fn from_f64s(values: Vec<Vec<f64>>) -> Option<Self> {
        Self::new(
            values
                .into_iter()
                .map(|row| row.into_iter().map(|v| v as f32).collect())
                .collect(),
        )
    }

    pub fn dim(&self) -> usize {
        self.values.len()
    }

    fn is_logical(&self) -> bool {
        self.values
            .iter()
            .all(|row| row.iter().all(|&v| v == 0.0 || v == 1.0))
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> Vec<Vec<Scalar>> {
        self.values
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Scalar;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[x][y]
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let logical = self.is_logical();
        writeln!(f)?;
        writeln!(f, "  {}", ('a'..='z').take(self.dim()).join(" "))?;
        for (row_index, row) in self.values.iter().enumerate() {
            write!(
                f,
                "{} ",
                ('a'..='z').nth(row_index).expect("27 cols a but much mate")
            )?;
            for (col_index, entry) in row.iter().enumerate() {
                if logical {
                    if row_index == col_index {
                        write!(f, "- ")?;
                    } else {
                        write!(f, "{entry:.0} ")?;
                    }
                } else {
                    if row_index == col_index {
                        write!(f, "  --  ")?;
                    } else {
                        write!(f, "{entry:.3} ")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
