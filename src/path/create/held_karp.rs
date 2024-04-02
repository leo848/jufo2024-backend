use std::iter::empty;

use crate::dist_graph::Point;
use crate::graph::Path;
use crate::path::{CreateContext, Matrix};
use crate::typed::Metric;
use crate::Graph;

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    use std::collections::{HashMap, HashSet};

    let matrix = ctx.adjacency_matrix();

    let size = matrix.dim();

    assert!(size < 32);

    let mut dp_memo: Box<[Box<[f32]>]> =
        vec![vec![f32::INFINITY; 1 << size].into_boxed_slice(); size].into_boxed_slice();
    dp_memo[0][1] = 0.0;

    for mask in 1..1 << size {
        if mask & ((1 << (size.saturating_sub(5))) - 1) == 0 {
            ctx.send_edges(empty(), Some(mask as f32 / (1 << size) as f32));
        }
        for last_node in 0..size {
            if (mask & (1 << last_node)) == 0 {
                continue;
            }
            for next_node in 0..size {
                if mask & 1 << next_node != 0 {
                    continue;
                }
                dp_memo[next_node][mask | (1 << next_node)] = f32::min(
                    dp_memo[next_node][mask | (1 << next_node)],
                    dp_memo[last_node][mask] + matrix[(last_node, next_node)],
                )
            }
        }
    }

    let mut min_chain_length = f32::INFINITY;
    let mut last_node = 0;
    for i in 0..size {
        let chain_length = dp_memo[i][(1 << size) - 1];
        if chain_length < min_chain_length {
            min_chain_length = chain_length;
            last_node = i;
        }
    }

    let mut path = vec![0; size];
    let mut mask = (1 << size) - 1;
    path[size - 1] = last_node;

    for i in (1..size).rev() {
        let mut next_node = 0;
        let mut min_chain_length = f32::INFINITY;
        for j in 0..size {
            if j != last_node && (mask & (1 << j)) != 0 {
                let cost = dp_memo[j][mask] + matrix[(j, last_node)];
                if cost < min_chain_length {
                    min_chain_length = cost;
                    next_node = j;
                }
            }
        }
        path[i - 1] = next_node;
        mask &= !(1 << last_node);
        last_node = next_node;
    }

    ctx.path_from_indices(path.iter().copied())
}
