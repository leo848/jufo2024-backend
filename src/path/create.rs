use crate::path::creation::PathCreation;
use std::collections::HashSet;
use core::ops::Not;
use itertools::Itertools;

use crate::{action::PathCreateContext, graph::Path};

pub fn transmute(ctx: PathCreateContext) -> Path {
    let PathCreateContext { action: _, graph } = ctx;
    Path::new(graph.node_indices().collect_vec())
}

pub fn random(ctx: PathCreateContext) -> Path {
    let PathCreateContext { action: _, graph } = ctx;
    let mut vec = graph.node_indices().collect_vec();
    fastrand::shuffle(&mut vec);
    Path::new(vec)
}

pub fn nearest_neighbor(ctx: PathCreateContext) -> Path {
    let PathCreateContext { action, graph } = ctx;

    let mut visited = HashSet::new();
    let mut path = Path::with_capacity(graph.size());
    path.push(0);
    while path.len() < graph.size() {
        let last = path[path.len() - 1];
        visited.insert(last.clone());

        let min = graph.node_indices()
            .filter(|ni| Not::not(visited.contains(ni)))
            .min_by_key(|&ni| graph.weight(ni, last))
            .expect("point was empty even though path is not full");

        path.push(min.clone());
        action.send(
            PathCreation::from_path(path.clone()).progress(path.len() as f32 / graph.node_indices().len() as f32),
        );
    }

    path
}
