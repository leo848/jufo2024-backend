use core::hash::Hash;

use itertools::Itertools;

pub mod create;
pub mod creation;
pub mod improve;

#[derive(PartialEq, Debug, Clone)]
pub struct HashPoint(pub Vec<f32>);

impl Hash for HashPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0
            .iter()
            .map(|comp| comp.to_bits())
            .for_each(|bits| state.write_u32(bits))
    }
}

impl Eq for HashPoint {}

pub fn distance_squared(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(comp1, comp2)| (comp1 - comp2) * (comp1 - comp2))
        .sum()
}

pub fn distance(a: &[f32], b: &[f32]) -> f32 {
    distance_squared(a, b).sqrt()
}

pub fn cost(values: &[Vec<f32>]) -> f32 {
    values.windows(2).map(|s| distance(&s[0], &s[1])).sum()
}

pub fn path_to_edges(path: &[Vec<f32>]) -> Vec<(Vec<f32>, Vec<f32>)> {
    path.iter().cloned().tuple_windows::<(_, _)>().collect()
}
