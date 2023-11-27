use core::hash::Hash;

pub mod create;
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
    return a
        .iter()
        .zip(b)
        .map(|(comp1, comp2)| (comp1 - comp2) * (comp1 - comp2))
        .sum();
}

pub fn cost(values: &[Vec<f32>]) -> f32 {
    values.windows(2).map(|s| distance_squared(&s[0], &s[1])).sum()
}
