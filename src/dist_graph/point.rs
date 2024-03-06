use core::hash::Hash;

use serde::Serialize;

use crate::{
    dist_graph::{Cost, Scalar},
    typed::{Metric, Norm},
};
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

    fn euclidean_dist_squared(&self, other: &Point) -> Cost {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(comp1, comp2)| (comp1 - comp2) * (comp1 - comp2))
            .sum()
    }

    fn euclidean_dist(&self, other: &Point) -> Cost {
        self.euclidean_dist_squared(other).sqrt()
    }

    fn manhattan_dist(&self, other: &Point) -> Cost {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(comp1, comp2)| (comp1 - comp2).abs())
            .sum()
    }

    fn max_dist(&self, other: &Point) -> Cost {
        Cost::new(
            self.0
                .iter()
                .zip(&other.0)
                .map(|(comp1, comp2)| (comp1 - comp2).abs())
                .max_by(Scalar::total_cmp)
                .unwrap_or(0.0),
        )
    }
    #[inline]
    pub fn dist(&self, other: &Point, metric: Metric) -> Cost {
        let normed = match metric.norm {
            Norm::Manhattan => self.manhattan_dist(other),
            Norm::Euclidean => self.euclidean_dist(other),
            Norm::Max => self.max_dist(other),
        };
        if metric.invert {
            -normed
        } else {
            normed
        }
    }
}

impl Eq for Point {}
