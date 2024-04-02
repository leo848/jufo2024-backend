use core::ops::Not;
use std::collections::HashSet;

use bimap::BiMap;
use itertools::Itertools;

use super::CreateContext;
use crate::{
    graph,
    util::{factorial, UsableFloat},
};

#[allow(unused)]
mod ilp;

#[allow(unused)]
mod held_karp;

pub use held_karp::solve as held_karp;
pub use ilp::solve as ilp;

pub fn transmute<C: CreateContext>(ctx: C) -> C::Path {
    ctx.path_from_indices(ctx.node_indices())
}

pub fn random<C: CreateContext>(ctx: C) -> C::Path {
    let mut path = ctx.node_indices().collect_vec();
    fastrand::shuffle(&mut path);
    ctx.path_from_indices(path)
}

pub fn optimal_nearest_neighbor<C: CreateContext>(ctx: C) -> C::Path {
    let mut best_path: Option<graph::Path> = None;

    for start_point in ctx.node_indices() {
        let mut visited = HashSet::new();
        let mut path = graph::Path::new(vec![start_point]);
        while path.len() != ctx.len() {
            let last = path[path.len() - 1];
            visited.insert(last.clone());

            let min = ctx
                .node_indices()
                .filter(|&ni| Not::not(visited.contains(&ni)))
                .min_by_key(|&ni| ctx.dist(last, ni).usable())
                .expect("point was empty even though path is not full");

            path.push(min.clone());
            if start_point == 0 {
                ctx.send_path(
                    path.iter(),
                    Some(path.len() as f32 / ctx.len() as f32 / 2.0),
                );
            }
        }

        ctx.send_path(
            path.iter(),
            Some(start_point as f32 / ctx.len() as f32 / 2.0 + 0.5),
        );
        if best_path
            .clone()
            .map(|best_path| ctx.cost(&path) < ctx.cost(&best_path))
            .unwrap_or(true)
        {
            best_path = Some(path);
        }
    }

    ctx.path_from_indices(best_path.expect("No path").iter())
}

pub fn brute_force<C: CreateContext>(ctx: C) -> C::Path {
    let mut min = f32::INFINITY;

    let permutation_count = factorial(ctx.len());
    let mut min_permutation = ctx.node_indices().collect_vec();

    let send_every = permutation_count.next_power_of_two() >> 5;

    for (i, permutation) in ctx.node_indices().permutations(ctx.len()).enumerate() {
        let cost = ctx.dist_path(permutation.iter().copied()).into();
        if cost < min {
            min = cost;
            min_permutation = permutation;
        }
        if ((i & (send_every - 1)) == 0) || cost < min {
            ctx.send_path(
                min_permutation.clone(),
                Some(i as f32 / permutation_count as f32),
            );
        }
    }

    ctx.path_from_indices(min_permutation)
}

pub fn nearest_neighbor<C: CreateContext>(ctx: C) -> C::Path {
    let mut visited = HashSet::new();
    let mut path = graph::Path::new(vec![0]);
    while path.len() != ctx.len() {
        let last = path[path.len() - 1];
        visited.insert(last.clone());

        let min = ctx
            .node_indices()
            .filter(|&ni| Not::not(visited.contains(&ni)))
            .min_by_key(|&ni| ctx.dist(last, ni).usable())
            .expect("point was empty even though path is not full");

        path.push(min.clone());
        ctx.send_path(path.iter(), Some(path.len() as f32 / ctx.len() as f32));
    }

    ctx.path_from_indices(path.iter())
}

pub fn greedy<C: CreateContext>(ctx: C) -> C::Path {
    let mut sorted_edge_iterator = ctx
        .node_indices()
        .cartesian_product(ctx.node_indices())
        .filter(|(l, r)| l != r)
        .sorted_by_key(|(l, r)| ctx.dist(*l, *r).usable());

    // BiMap von Knoten
    let mut bimap = BiMap::with_capacity(ctx.len());
    let mut separate_list = Vec::<graph::Edge>::new();

    'outer: while bimap.len() < ctx.len() - 1 {
        let next_try = sorted_edge_iterator
            .next()
            .expect("there should be edges left");

        let insert = bimap.insert_no_overwrite(next_try.0, next_try.1);
        if insert.is_err() {
            continue;
        } else {
            separate_list.push(graph::Edge::new(next_try.0, next_try.1));
        }

        // Ist next_try.0 Teil eines Zyklus? Falls ja, vorab abbrechen.
        let mut element = next_try.0;
        while let Some(&next) = bimap.get_by_left(&element) {
            if next == next_try.0 {
                // Einfügen rückgängig machen
                bimap.remove_by_left(&next_try.0);
                separate_list.pop();
                continue 'outer;
            }
            element = next;
        }

        ctx.send_edges(
            separate_list.iter().map(|&edge| (edge.0, edge.1)),
            Some(bimap.len() as f32 / ctx.len() as f32),
        );
    }

    let mut path: graph::Path = graph::Path::with_capacity(ctx.len());
    let mut start = 0;
    while let Some(&from) = bimap.get_by_right(&start) {
        start = from;
    }
    path.push(start);
    while let Some(to) = bimap.get_by_left(&path[path.len() - 1]) {
        path.push(to.clone());
    }

    assert_eq!(ctx.len(), path.len());

    ctx.path_from_indices(path.iter())
}

pub fn insertion<C: CreateContext>(ctx: C) -> C::Path {
    let mut visited = HashSet::new();
    let mut path = graph::Path::new(vec![0]);
    while path.len() != ctx.len() {
        let mut min_cost_delta = f32::INFINITY;
        let mut min_action = None;
        for new_vertex in ctx
            .node_indices()
            .filter(|&ni| Not::not(visited.contains(&ni)))
        {
            for i in 0..=path.len() {
                let cost_delta = if i == 0 {
                    ctx.dist(new_vertex, 0)
                } else if i == path.len() {
                    ctx.dist(path[path.len() - 1], new_vertex)
                } else {
                    ctx.dist(path[i - 1], new_vertex) + ctx.dist(new_vertex, path[i])
                        - ctx.dist(path[i - 1], path[i])
                };
                if cost_delta < min_cost_delta {
                    min_cost_delta = cost_delta;
                    min_action = Some((i, new_vertex));
                }
            }
        }
        let Some((i, new_vertex)) = min_action else {
            panic!("invalid configuration");
        };
        path.insert(i, new_vertex);
        visited.insert(new_vertex);
        ctx.send_path(path.iter(), Some(path.len() as f32 / ctx.len() as f32));
    }

    ctx.path_from_indices(path.iter())
}

// pub fn christofides(ctx: DistPathCreateContext) -> Path {
//     let DistPathCreateContext {
//         action,
//         points: values,
//         dim: _,
//         norm,
//     } = ctx;

//     // 1. Finde den MST (minimalen Baum, der alle Knoten verbindet)
//     let mut visited = HashSet::new();
//     let mut edges = Edges::new();

//     let first_vertex = values[0].clone();
//     visited.insert(first_vertex);

//     while visited.len() < values.len() {
//         let min_edge = values
//             .edges_iter()
//             .filter(|edge| {
//                 visited.contains(edge.from()) != visited.contains(edge.to()) // einer von beiden
//             })
//             .min_by_key(|e| e.comparable_dist(norm))
//             .expect("no edges");

//         visited.insert(min_edge.from().clone());
//         visited.insert(min_edge.to().clone());
//         edges.push(min_edge);

//         action.send(DistPathCreation::from_edges(edges.clone()))
//     }

//     let mst = edges;

//     // // 2. Finde eine perfekte Paarung im Teilgraph aller Kanten ungeraden Grades

//     let odd_degree_vertices: Points = visited
//         .clone()
//         .into_iter()
//         .map(|point| {
//             (
//                 point.clone(),
//                 mst.clone()
//                     .into_iter()
//                     .filter(|contained_edge| {
//                         contained_edge.from() == &point || contained_edge.to() == &point
//                     })
//                     .count(),
//             )
//         })
//         .filter(|(_, degree)| degree % 2 == 1)
//         .map(|(point, _)| point)
//         .collect();

//     let _odd_degree_edges = odd_degree_vertices.edges_iter().collect::<HashSet<_>>();

//     todo!()
// }
