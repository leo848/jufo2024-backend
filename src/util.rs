#![allow(clippy::range_plus_one)]
pub fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}
