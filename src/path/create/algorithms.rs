use core::ops::Not;
use std::collections::HashSet;

use bimap::BiMap;
use itertools::Itertools;
use simple_websockets::Responder;

use super::{distance_squared, edges};
use crate::{
    path::{cost, creation::PathCreation, HashPoint},
    typed::send,
    util::factorial,
};

#[inline]
pub fn assert_dim(dim: u8, values: &[Vec<f32>]) {
    assert!(values.iter().all(|s| s.len() == dim as usize))
}

pub fn nearest_neighbor(client: &Responder, dim: u8, values: &mut Vec<Vec<f32>>) {
    assert_dim(dim, values);

    // let sleep_time = u64::min(values.len() as u64 * 500, 5000) / values.len() as u64;

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
        send(
            client,
            PathCreation::from_path(path.clone()).progress(path.len() as f32 / values.len() as f32),
        );
    }

    *values = path;
}

pub fn brute_force(client: &Responder, dim: u8, values: &mut Vec<Vec<f32>>) {
    assert_dim(dim, values);

    let mut min = f32::INFINITY;
    let mut min_permutation = values.clone();

    let permutation_count = factorial(values.len());

    for (i, permutation) in values
        .clone()
        .into_iter()
        .permutations(values.len())
        .enumerate()
    {
        send(
            client,
            PathCreation::from_path(permutation.clone())
                .progress(i as f32 / permutation_count as f32),
        );
        if cost(&permutation) < min {
            min = cost(&permutation);
            min_permutation = permutation;
        }
    }

    *values = min_permutation;
}

pub fn greedy<'a>(client: &Responder, dim: u8, values: &'a mut Vec<Vec<f32>>) {
    assert_dim(dim, &values);

    let mut sorted_edge_iterator = edges(&values)
        .sorted_by(|(f1, t1), (f2, t2)| {
            distance_squared(f1, t1).total_cmp(&distance_squared(f2, t2))
        })
        .into_iter();

    let mut bimap = BiMap::new();

    'outer: while bimap.len() < values.len() - 1 {
        let next_try = sorted_edge_iterator
            .next()
            .expect("there should be edges left");

        let insert =
            bimap.insert_no_overwrite(HashPoint(next_try.0.clone()), HashPoint(next_try.1.clone()));
        if insert.is_err() {
            continue;
        }

        // Ist next_try.0 Teil eines Zyklus? Falls ja, vorab abbrechen.
        let mut element = HashPoint(next_try.0.clone());
        while let Some(next) = bimap.get_by_left(&element) {
            if next == &HashPoint(next_try.0.clone()) {
                // Einfügen rückgängig machen
                bimap.remove_by_left(&HashPoint(next_try.0.clone()));
                continue 'outer;
            }
            element = next.clone();
        }

        send(
            client,
            PathCreation::from_edges(
                bimap
                    .iter()
                    .map(|t| (t.0.clone().0, t.1.clone().0))
                    .collect(),
            )
            .progress(bimap.len() as f32 / values.len() as f32),
        );
    }

    let mut path: Vec<Vec<f32>> = Vec::with_capacity(values.len());
    let mut min = HashPoint(values[0].clone());
    while let Some(from) = bimap.get_by_right(&min) {
        min = from.clone();
    }
    path.push(min.0);
    while let Some(to) =
        bimap.get_by_left(&HashPoint(path.last().expect("no item removal").clone()))
    {
        path.push(to.0.clone());
    }

    assert_eq!(values.len(), path.len());

    *values = path;
}
