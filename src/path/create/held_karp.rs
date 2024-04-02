use std::iter::empty;

use itertools::Itertools;

use crate::{
    dist_graph::Point,
    graph::Path,
    path::{CreateContext, Matrix},
    typed::Metric,
    Graph,
};

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    use std::collections::{HashMap, HashSet};

    let size = ctx.len();

    assert!(size < 32);

    let mut global_best_path = None;
    let mut global_best_chain_length = f32::INFINITY;

    for start_point in ctx.node_indices() {
        let local_ctx = ctx.clone().rotate_left(start_point);
        let matrix = local_ctx.adjacency_matrix();

        let mut dp_memo: Box<[Box<[f32]>]> =
            vec![vec![f32::INFINITY; size].into_boxed_slice(); 1 << size].into_boxed_slice();

        dp_memo[1 << 0][0] = 0.0;

        for mask in 1..1 << size {
            if mask & ((1 << (size.saturating_sub(2))) - 1) == 0 {
                ctx.send_path(
                    global_best_path.iter().flatten().copied(),
                    Some(
                        ((mask / (1 << size)) as f32) / local_ctx.len() as f32
                            + (start_point as f32 / local_ctx.len() as f32),
                    ),
                );
            }
            for last_node in 0..size {
                if (mask & (1 << last_node)) == 0 {
                    continue;
                }
                for next_node in 0..size {
                    if (mask & (1 << next_node)) != 0 {
                        continue;
                    }
                    dp_memo[mask | (1 << next_node)][next_node] = f32::min(
                        dp_memo[mask | (1 << next_node)][next_node],
                        dp_memo[mask][last_node] + matrix[(last_node, next_node)],
                    )
                }
            }
        }

        let mut min_chain_length = f32::INFINITY;
        let mut last_node = 0;
        let mut best_path = vec![];
        let mut best_chain_length = f32::INFINITY;

        for last_node in 0..size {
            let mut path = vec![0; size];
            let mut mask = (1 << size) - 1;
            path[size - 1] = last_node;

            let mut new_last_node = last_node;
            for i in (0..size - 1).rev() {
                let mut next_node = 0;
                let mut min_chain_length = f32::INFINITY;
                for j in 0..size {
                    if j != new_last_node && (mask & (1 << j)) != 0 {
                        let cost = dp_memo[mask][j] + matrix[(j, new_last_node)];
                        if cost < min_chain_length {
                            min_chain_length = cost;
                            next_node = j;
                        }
                    }
                }
                path[i] = next_node;
                mask &= !(1 << new_last_node);
                new_last_node = next_node;
            }

            if path.iter().all_unique() {
                let path_chain_length = local_ctx.dist_path(path.iter().copied());
                if path_chain_length < best_chain_length {
                    best_chain_length = path_chain_length;
                    best_path = path.clone();
                }
            }
        }

        if best_chain_length < global_best_chain_length {
            global_best_chain_length = best_chain_length;
            global_best_path = Some(
                best_path
                    .into_iter()
                    .map(|index| {
                        (((index as isize) + (start_point as isize)).rem_euclid(ctx.len() as isize))
                            as usize
                    })
                    .collect_vec(),
            );
        }
        ctx.send_path(
            global_best_path
                .as_ref()
                .expect("just set the value")
                .iter()
                .copied(),
            Some(start_point as f32 / ctx.len() as f32),
        );
    }

    ctx.path_from_indices(global_best_path.expect("No path found").iter().copied())
}
