use core::ops::Not;
use std::collections::HashSet;

use bimap::BiMap;
use itertools::Itertools;

use crate::{
    action::PathCreateContext,
    graph::{Cost, Edge, Edges, Path, Points},
    path::creation::PathCreation,
    util::factorial,
};

pub fn random(ctx: PathCreateContext) -> Path {
    let values = ctx.points;
    let mut path = values.into_path();
    fastrand::shuffle(path.as_mut());
    path
}

pub fn transmute(ctx: PathCreateContext) -> Path {
    let values = ctx.points;
    values.into_path()
}

pub fn nearest_neighbor(ctx: PathCreateContext) -> Path {
    let PathCreateContext {
        action,
        dim,
        points: values,
    } = ctx;

    let mut visited = HashSet::new();
    let mut path = Path::try_new(vec![values[0].clone()], dim).expect("Provided valid value");
    while path.len() != values.len() {
        let last = &path[path.len() - 1];
        visited.insert(last.clone());

        let min = values
            .iter()
            .filter(|&point| Not::not(visited.contains(point)))
            .min_by_key(|point| point.dist_squared(last))
            .expect("point was empty even though path is not full");

        path.push(min.clone());
        action.send(
            PathCreation::from_path(path.clone()).progress(path.len() as f32 / values.len() as f32),
        );
    }

    path
}

pub fn brute_force(ctx: PathCreateContext) -> Path {
    let PathCreateContext {
        action,
        points: values,
        ..
    } = ctx;

    let mut min = Cost::new(f32::INFINITY);

    let permutation_count = factorial(values.len());
    let mut min_permutation = values.clone();

    let send_every = permutation_count.next_power_of_two() >> 5;

    for (i, permutation) in values.permutations().enumerate() {
        let path = permutation.clone().into_path();
        let cost = path.cost();
        if cost < min {
            min = cost;
            min_permutation = permutation;
        }
        if ((i & (send_every - 1)) == 0) || cost < min {
            action.send(
                PathCreation::from_path(min_permutation.clone().into_path())
                    .progress(i as f32 / permutation_count as f32),
            );
        }
    }

    min_permutation.into_path()
}

pub fn greedy(ctx: PathCreateContext) -> Path {
    let PathCreateContext {
        action,
        points: values,
        ..
    } = ctx;

    let mut sorted_edge_iterator = values.edges_iter().sorted_by_key(Edge::dist_squared);

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
        while let Some(next) = bimap.get_by_left(element) {
            if next == next_try.from() {
                // Einfügen rückgängig machen
                bimap.remove_by_left(next_try.from());
                continue 'outer;
            }
            element = next;
        }

        action.send(
            PathCreation::from_edges(Edges::from_bimap(bimap.clone()))
                .progress(bimap.len() as f32 / values.len() as f32),
        );
    }

    let mut path: Path = Path::with_capacity(values.len());
    let mut min = &values[0];
    while let Some(from) = bimap.get_by_right(min) {
        min = from;
    }
    path.push(min.clone());
    while let Some(to) = bimap.get_by_left(&path[path.len() - 1]) {
        path.push(to.clone());
    }

    assert_eq!(values.len(), path.len());

    path
}

pub fn christofides(ctx: PathCreateContext) -> Path {
    let PathCreateContext {
        action,
        points: values,
        ..
    } = ctx;

    // 1. Finde den MST (minimalen Baum, der alle Knoten verbindet)
    let mut visited = HashSet::new();
    let mut edges = Edges::new();

    let first_vertex = values[0].clone();
    visited.insert(first_vertex);

    while visited.len() < values.len() {
        let min_edge = values
            .edges_iter()
            .filter(|edge| {
                visited.contains(edge.from()) != visited.contains(edge.to()) // einer von beiden
            })
            .min_by_key(Edge::dist_squared)
            .expect("no edges");

        visited.insert(min_edge.from().clone());
        visited.insert(min_edge.to().clone());
        edges.push(min_edge);

        action.send(PathCreation::from_edges(edges.clone()))
    }

    let mst = edges;

    // // 2. Finde eine perfekte Paarung im Teilgraph aller Kanten ungeraden Grades

    let odd_degree_vertices: Points = visited
        .clone()
        .into_iter()
        .map(|point| {
            (
                point.clone(),
                mst.clone()
                    .into_iter()
                    .filter(|contained_edge| {
                        contained_edge.from() == &point || contained_edge.to() == &point
                    })
                    .count(),
            )
        })
        .filter(|(_, degree)| degree % 2 == 1)
        .map(|(point, _)| point)
        .collect();

    let _odd_degree_edges = odd_degree_vertices.edges_iter().collect::<HashSet<_>>();

    todo!()
}
