use core::ops::Not;
use std::collections::HashSet;

use bimap::BiMap;
use itertools::Itertools;
use simple_websockets::Responder;

use crate::{
    graph::{Cost, Edges, Path, Points},
    path::creation::PathCreation,
    typed::send,
    util::factorial,
};

pub fn random(_client: &Responder, _dim: u8, values: Points) -> Path {
    let mut path = values.as_path();
    fastrand::shuffle(&mut path.as_mut());
    path
}

pub fn transmute(_client: &Responder, _dim: u8, values: Points) -> Path {
    values.as_path()
}

pub fn nearest_neighbor(client: &Responder, dim: u8, values: Points) -> Path {
    // let sleep_time = u64::min(values.len() as u64 * 500, 5000) / values.len() as u64;

    let mut visited = HashSet::new();
    let mut path = Path::try_new(vec![values[0].clone()], dim).expect("Provided valid value");
    while path.len() != values.len() {
        let last = &path[path.len() - 1];
        visited.insert(last.clone());

        let min = values
            .iter()
            .filter(|&point| Not::not(visited.contains(&point)))
            .min_by_key(|point| point.dist_squared(&last))
            .expect("point was empty even though path is not full");

        path.push(min.clone());
        send(
            client,
            PathCreation::from_path(path.clone()).progress(path.len() as f32 / values.len() as f32),
        );
    }

    path
}

pub fn brute_force(client: &Responder, _dim: u8, values: Points) -> Path {
    let mut min = Cost::new(f32::INFINITY);

    let permutation_count = factorial(values.len());
    let mut min_permutation = values.clone();

    let send_every = permutation_count.next_power_of_two() >> 5;

    for (i, permutation) in values.permutations().enumerate() {
        let path = permutation.clone().as_path();
        let cost = path.cost();
        if cost < min {
            min = cost;
            min_permutation = permutation;
        }
        if ((i & (send_every - 1)) == 0) || cost < min {
            send(
                client,
                PathCreation::from_path(min_permutation.clone().as_path())
                    .progress(i as f32 / permutation_count as f32),
            );
        }
    }

    min_permutation.as_path()
}

pub fn greedy<'a>(client: &Responder, _dim: u8, values: Points) -> Path {
    let mut sorted_edge_iterator = values
        .edges_iter()
        .sorted_by_key(|edge1| edge1.dist_squared())
        .into_iter();

    let mut bimap = BiMap::new();

    'outer: while bimap.len() < values.len() - 1 {
        let next_try = sorted_edge_iterator
            .next()
            .expect("there should be edges left");

        let insert = bimap.insert_no_overwrite(next_try.from().clone(), next_try.to().clone());
        if insert.is_err() {
            continue;
        }

        // Ist next_try.0 Teil eines Zyklus? Falls ja, vorab abbrechen.
        let mut element = next_try.from();
        while let Some(next) = bimap.get_by_left(&element) {
            if next == next_try.from() {
                // Einfügen rückgängig machen
                bimap.remove_by_left(next_try.from());
                continue 'outer;
            }
            element = next;
        }

        send(
            client,
            PathCreation::from_edges(Edges::from_bimap(bimap.clone()))
                .progress(bimap.len() as f32 / values.len() as f32),
        );
    }

    let mut path: Path = Path::with_capacity(values.len());
    let mut min = &values[0];
    while let Some(from) = bimap.get_by_right(&min) {
        min = from;
    }
    path.push(min.clone());
    while let Some(to) = bimap.get_by_left(&path[path.len() - 1]) {
        path.push(to.clone());
    }

    assert_eq!(values.len(), path.len());

    path
}
