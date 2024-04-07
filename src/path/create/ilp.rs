use serde::Deserialize;
use std::iter::once;

use crate::path::Matrix;
use crate::CreateContext;
use coin_cbc::Col;
use coin_cbc::{Model, Sense};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MilpSolver {
    #[default]
    CoinOrCbc,
}

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    let matrix: Matrix = ctx.adjacency_matrix().normalize().scale(100.0);

    let size = ctx.len();
    let node_indices = { || (0..size) };

    let mut model = Model::default();
    model.set_parameter("messages", "off");

    model.set_obj_sense(Sense::Minimize);

    let x: Vec<Vec<Col>> = {
        node_indices()
            .map(|_| node_indices().map(|_| model.add_binary()).collect_vec())
            .collect_vec()
    };

    // let u: Vec<Col> = node_indices().map(|i| model.add_integer()).collect_vec();

    for i in node_indices() {
        for j in node_indices() {
            model.set_obj_coeff(x[i][j], matrix[(i, j)].into())
        }
    }

    // Jede Zeile darf maximal einen Eintrag haben.
    for i in node_indices() {
        let constraint = model.add_row();
        model.set_row_upper(constraint, 1.0);
        for j in node_indices() {
            model.set_weight(constraint, x[i][j], 1.0);
        }
    }

    // Jede Spalte darf maximal einen Eintrag haben.
    for j in node_indices() {
        let constraint = model.add_row();
        model.set_row_upper(constraint, 1.0);
        for i in node_indices() {
            model.set_weight(constraint, x[i][j], 1.0);
        }
    }

    // Jeder Knoten muss erreicht werden.
    for i in node_indices() {
        let mut pairs = Vec::new();
        for j in node_indices() {
            pairs.push((i, j));
            if i != j {
                pairs.push((j, i));
            }
        }
        pairs.sort_unstable();

        let constraint = model.add_row();
        model.set_row_lower(constraint, 1.0);
        for (i, j) in pairs {
            model.set_weight(constraint, x[i][j], 1.0);
        }
    }

    // Keine Hin- und Rückkanten
    for i in node_indices() {
        for j in node_indices().skip(i) {
            let constraint = model.add_row();
            if i == j {
                model.set_row_equal(constraint, 0.0);
                model.set_weight(constraint, x[i][i], 1.0);
            } else {
                model.set_row_upper(constraint, 1.0);
                model.set_weight(constraint, x[i][j], 1.0);
                model.set_weight(constraint, x[j][i], 1.0);
            }
        }
    }

    // Insgesamt werden n-1 Knoten erreicht.
    let constraint = model.add_row();
    model.set_row_equal(constraint, (size - 1) as f64);
    for i in node_indices() {
        for j in node_indices() {
            model.set_weight(constraint, x[i][j], 1.0);
        }
    }

    // Zyklen verhindern

    model.solve();

    let mut paths = print_paths_solution(&mut model, &x);

    while paths.len() > 1 {
        let edges_to_send = paths
            .iter()
            .flat_map(|path| {
                path.iter()
                    .tuple_windows::<(_, _)>()
                    .chain(if path[0] == path[path.len() - 1] {
                        vec![(&path[path.len() - 1], &path[0])]
                    } else {
                        vec![]
                    })
            })
            .map(|(&i, &j)| (i, j))
            .filter(|(i, j)| i != j)
            .sorted()
            .dedup();
        ctx.send_edges(edges_to_send, None);
        add_cycle_constraints(&mut model, &paths, &x);
        paths = print_paths_solution(&mut model, &x);
    }

    let path = &paths[0];

    // (matrix[(i, j)].into(), (0.0, 1.0));

    ctx.path_from_indices(path.iter().copied())
}

fn print_paths_solution(model: &mut Model, x: &Vec<Vec<Col>>) -> Vec<Vec<usize>> {
    let sol = model.solve();
    let paths = edges_to_paths({
        let sol = &sol;
        let x = &x;
        &(0..x.len())
            .flat_map(move |i| {
                (0..x.len())
                    .map(move |j| (i, j, sol.col(x[i][j])))
                    .filter(|&(_, _, v)| v == 1.0)
                    .map(|(i, j, _)| (i, j))
            })
            .collect_vec()
    });
    for path in paths.iter() {
        println!("{}", path.iter().join(" -> "));
    }
    println!();
    paths
}

fn add_cycle_constraints(model: &mut Model, paths: &[Vec<usize>], x: &Vec<Vec<Col>>) {
    let is_cycle = |p: &[usize]| p[0] == p[p.len() - 1];
    for path in paths {
        let row = model.add_row();
        model.set_row_upper(row, path.len() as f64 - 2.0);
        if !is_cycle(path) {
            continue;
        }
        let edges = cycle_to_edges_symmetric(&path[1..]);
        for (i, j) in edges {
            model.set_weight(row, x[i][j], 1.0);
            model.set_weight(row, x[j][i], 1.0);
        }
    }
}

fn edges_to_paths(edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut paths: Vec<Vec<usize>> = Vec::with_capacity(edges.len());
    for &(i, j) in edges {
        let mut inserted = false;
        'insertion: for path in paths.iter_mut() {
            for (index, node) in path.clone().iter().copied().enumerate() {
                if node == i {
                    path.insert(index + 1, j);
                    inserted = true;
                    break 'insertion;
                } else if node == j {
                    path.insert(index, i);
                    inserted = true;
                    break 'insertion;
                }
            }
        }
        if !inserted {
            paths.push(vec![i, j]);
        }
    }
    let mut changed = true;
    'outer: while changed {
        changed = false;
        paths.retain(|path| path.len() >= 1);
        for paths_idx in 0..paths.len() {
            for other_paths_idx in paths_idx + 1..paths.len() {
                let path = &paths[paths_idx];
                let other_path = &paths[other_paths_idx];
                if path[path.len() - 1] == other_path[0] {
                    let other_iter = other_path.clone();
                    paths[paths_idx].extend(&other_iter[1..]);
                    paths[other_paths_idx].clear();
                    changed = true;
                    continue 'outer;
                } else if other_path[other_path.len() - 1] == path[0] {
                    let iter = path.clone();
                    paths[other_paths_idx].extend(&iter[1..]);
                    paths[paths_idx].clear();
                    changed = true;
                    continue 'outer;
                }
            }
        }
    }
    paths
}

fn cycle_to_edges(cycle: &[usize]) -> impl Iterator<Item = (usize, usize)> + '_ {
    cycle
        .windows(2)
        .map(|t| (t[0], t[1]))
        .chain(once((cycle[cycle.len() - 1], cycle[0])))
}

fn cycle_to_edges_symmetric(cycle: &[usize]) -> impl Iterator<Item = (usize, usize)> + '_ {
    cycle_to_edges(cycle).chain(cycle_to_edges(cycle).map(|(a, b)| (b, a)))
}
