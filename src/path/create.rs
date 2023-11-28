use itertools::Itertools;

use super::distance_squared;
mod algorithms;

pub use algorithms::*;

fn edges<'a>(values: &'a Vec<Vec<f32>>) -> impl Iterator<Item = (&'a Vec<f32>, &'a Vec<f32>)> + 'a {
    values.iter().cartesian_product(values).filter(|(t, u)| t != u)
}
