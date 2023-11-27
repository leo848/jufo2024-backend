use core::ops::Not;
use crate::path::HashPoint;
use crate::path::cost;
use crate::sleep_ms;
use std::collections::HashSet;

use crate::typed;
use crate::Output;
use simple_websockets::Responder;
use itertools::Itertools;

use super::distance_squared;

#[inline]
pub fn assert_dim(dim: u8, values: &[Vec<f32>]) {
    assert!(values.iter().all(|s| s.len() == dim as usize))
}

fn send(client: &Responder, values: &[Vec<f32>], sleep: u64) {
    typed::send(
        &client,
        Output::PathCreation {
            done: false,
            current_path: values.into(),
        },
    );
    sleep_ms(sleep);
}

pub fn nearest_neighbor(client: &Responder, dim: u8, values: &mut Vec<Vec<f32>>) {
    assert_dim(dim, values);

    let sleep_time = u64::min(values.len() as u64 * 500, 5000) / values.len() as u64;

    let mut visited = HashSet::new();
    let mut path = vec![values[0].clone()];
    while path.len() != values.len() {
        let last = &path[path.len() - 1];
        visited.insert(HashPoint(last.clone()));

        let min = values
            .iter()
            .filter(|&point| Not::not(visited.contains(&HashPoint(point.clone()))))
            .min_by(|point1, point2| {
                distance_squared(point1, &last).total_cmp(&distance_squared(point2, &last))
            })
            .expect("point was empty even though path is not full");

        path.push(min.clone());
        send(client, &path, sleep_time);
    }

    *values = path;
}

pub fn brute_force(client: &Responder, dim: u8, values: &mut Vec<Vec<f32>>) {
    assert_dim(dim, values);

    let mut min = f32::INFINITY;
    let mut min_permutation = values.clone();

    for permutation in values.clone().into_iter().permutations(values.len()) {
        send(client, &permutation, 0);
        if cost(&permutation) < min {
            min = cost(&permutation);
            min_permutation = permutation;
        }
    }

    *values = min_permutation;
}
