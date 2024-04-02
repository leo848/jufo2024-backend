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

    let matrix = ctx.adjacency_matrix();

    let size = ctx.len();

    assert!(size < 32);

    let mut dp_memo: Box<[Box<[f32]>]> =
        vec![vec![f32::INFINITY; size].into_boxed_slice(); 1 << size].into_boxed_slice();

    for i in 0..size {
        dp_memo[1 << i][i] = 0.0;
    }

    for mask in 1..1 << size {
        if mask & ((1 << (size.saturating_sub(5))) - 1) == 0 {
            ctx.send_edges(empty(), Some(0.9 * mask as f32 / (1 << size) as f32));
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
        // let chain_length = dp_memo[i][(1 << size) - 1];
        // if chain_length < min_chain_length {
        //     min_chain_length = chain_length;
        //     last_node = i;
        // }
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
            let path_chain_length = ctx.dist_path(path.iter().copied());
            ctx.send_path(
                path.iter().copied(),
                Some(0.9 + last_node as f32 / size as f32 * 0.1),
            );
            if path_chain_length < best_chain_length {
                best_chain_length = path_chain_length;
                best_path = path.clone();
            }
        }
    }

    ctx.path_from_indices(best_path)
}
