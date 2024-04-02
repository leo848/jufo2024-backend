use itertools::Itertools;
use minilp::{LinearExpr, Solution, Variable};

#[cfg(test)]
use crate::{
    dist_graph::Point,
    graph::{Graph, Path},
    typed::Metric,
};
use crate::{graph::Matrix, path::CreateContext};

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    let matrix: Matrix = ctx.adjacency_matrix();

    dbg!(matrix);

    todo!()
}

#[cfg(test)]
#[allow(unused)]
fn solve_simple(matrix: Matrix, names: &[String]) -> Path {
    use std::{array, iter};

    use bimap::BiMap;
    use itertools::Itertools;
    use minilp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

    dbg!(&matrix);

    let n = matrix.dim();
    let node_indices = { || (0..n) };

    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let variables = {
        node_indices()
            .map(|i| {
                node_indices()
                    .map(|j| problem.add_var(matrix[(i, j)].into(), (0.0, 1.0)))
                    .collect_vec()
            })
            .collect_vec()
    };

    // Jede Zeile darf maximal einen Eintrag haben.
    for i in 0..n {
        let mut row_sum = LinearExpr::empty();
        for j in 0..n {
            row_sum.add(variables[i][j], 1.0);
        }
        problem.add_constraint(row_sum, ComparisonOp::Le, 1.0);
    }

    // Jede Spalte darf maximal einen Eintrag haben.
    for j in 0..n {
        let mut col_sum = LinearExpr::empty();
        for i in 0..n {
            col_sum.add(variables[i][j], 1.0);
        }
        problem.add_constraint(col_sum, ComparisonOp::Le, 1.0);
    }

    // Jeder Knoten muss erreicht werden.
    for i in 0..n {
        let mut pairs = Vec::new();
        for j in 0..n {
            pairs.push((i, j));
            if i != j {
                pairs.push((j, i));
            }
        }
        pairs.sort_unstable();

        let vertex_sum = LinearExpr::from(pairs.into_iter().map(|(i, j)| (variables[i][j], 1.0)));
        problem.add_constraint(vertex_sum, ComparisonOp::Ge, 1.0);
    }

    // Keine Hin- und Rückkanten.
    for i in 0..n {
        for j in i..n {
            if i == j {
                problem.add_constraint(
                    LinearExpr::from([(variables[i][j], 1.0)]),
                    ComparisonOp::Le,
                    0.0,
                );
            } else {
                let symmetric = LinearExpr::from([(variables[i][j], 1.0), (variables[j][i], 1.0)]);
                problem.add_constraint(symmetric, ComparisonOp::Le, 1.0);
            }
        }
    }

    // Insgesamt werden n-1 Knoten erreicht.
    let mut total_sum = LinearExpr::empty();
    for i in 0..n {
        for j in 0..n {
            total_sum.add(variables[i][j], 1.0);
        }
    }
    problem.add_constraint(total_sum, ComparisonOp::Eq, n as f64 - 1.0);

    dbg!(&problem);

    let mut relaxed_solution = problem.solve().expect("nO solution found");

    // dbg!(&matrix);
    dbg!(&relaxed_solution);

    dbg!("adding cycle constraints");

    relaxed_solution = add_cycle_constraints(relaxed_solution, &variables);

    println!("{:?}", solution_to_matrix(&relaxed_solution, &variables));

    let mut edges = BiMap::new();
    for i in 0..n {
        for j in 0..n {
            if relaxed_solution[variables[i][j]] != 0.0 {
                edges.insert(names[i].clone(), names[j].clone());
            }
        }
    }

    let mut first = edges.iter().next().unwrap().0;
    while let Some(left) = edges.get_by_right(first) {
        // println!("first is {}", left);
        first = left;
    }
    let mut path = vec![first];
    while let Some(right) = edges.get_by_left(*path.last().unwrap()) {
        path.push(right);
    }
    println!("{}", path.iter().join(" -> "));

    // a -> g
    // b -> h
    // c -> i
    // d -> e
    // e -> f
    // f -> b
    // h -> d
    // i -> j
    // j -> c

    todo!();
}

fn solution_to_matrix(solution: &Solution, variables: &[Vec<Variable>]) -> Matrix {
    let n = variables.len();
    Matrix::from_f64s(
        (0..n)
            .map(|i| (0..n).map(|j| solution[variables[i][j]]).collect_vec())
            .collect_vec(),
    )
    .unwrap()
}

fn add_cycle_constraints(mut cur_solution: Solution, variables: &[Vec<Variable>]) -> Solution {
    let n = variables.len();
    let mut weights = Vec::with_capacity(n * n);
    loop {
        weights.clear();
        weights.resize(n * n, 0.0);
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    weights[i * n + j] = cur_solution[variables[i][j]];
                }
            }
        }
        // println!();
        // for chunk in weights.chunks(n) {
        //     println!("{}", chunk.iter().map(|n| format!("{:.0}", n)).join(" "));
        // }

        let (cut_weight, cut_mask) = find_min_cut(n, &mut weights);
        dbg!(cut_weight, &cut_mask);
        if cut_weight > 2.0 - 1e-8 || cut_mask.iter().filter(|&&b| b).count() == 1 {
            return cur_solution;
        }
        let mut cut_edges_sum = LinearExpr::empty();
        for i in 0..n {
            for j in 0..i {
                if cut_mask[i] != cut_mask[j] {
                    cut_edges_sum.add(variables[i][j], 1.0);
                }
            }
        }

        cur_solution = cur_solution
            .add_constraint(cut_edges_sum, minilp::ComparisonOp::Ge, 2.0)
            .expect("solution");

        // dbg!(&cur_solution);
    }
}

// Adaptiert aus: https://github.com/ztlpn/minilp/blob/master/examples/tsp.rs
fn find_min_cut(size: usize, weights: &mut [f64]) -> (f64, Vec<bool>) {
    assert!(size >= 2);
    assert_eq!(weights.len(), size * size);

    let mut is_merged = vec![false; size];
    let mut next_in_cluster = (0..size).collect_vec();

    let mut is_added = Vec::with_capacity(size);
    let mut node_weights = Vec::with_capacity(size);

    let mut best_cut_weight = f64::INFINITY;
    let mut best_cut = Vec::with_capacity(size);

    for i_phase in 0..(size - 1) {
        is_added.clear();
        is_added.extend_from_slice(&is_merged);

        node_weights.clear();
        node_weights.extend_from_slice(&weights[0..size]);

        let mut prev_node = 0;
        let mut last_node = 0;

        for _ in 0..(size - i_phase) {
            prev_node = last_node;
            let mut max_weight = f64::NEG_INFINITY;
            for n in 1..size {
                if !is_added[n] && node_weights[n] > max_weight {
                    last_node = n;
                    max_weight = node_weights[n];
                }
            }

            is_added[last_node] = true;
            for i in 0..size {
                if !is_added[i] {
                    node_weights[i] += weights[i * size + last_node];
                }
            }
        }

        let cut_weight = node_weights[last_node];
        let is_best_cut = cut_weight < best_cut_weight;
        if is_best_cut {
            best_cut_weight = cut_weight;
            best_cut.clear();
            best_cut.resize(size, false);
        }

        is_merged[prev_node] = true;
        let mut list_elem = last_node;
        loop {
            if is_best_cut {
                best_cut[list_elem] = true;
            }

            if next_in_cluster[list_elem] != list_elem {
                list_elem = next_in_cluster[list_elem];
            } else {
                next_in_cluster[list_elem] = prev_node;
                break;
            }
        }

        for n in 0..size {
            weights[last_node * size + n] += weights[prev_node * size + n];
        }
        for n in 0..size {
            weights[n * size + last_node] = weights[last_node * size + n];
        }
    }

    assert!(best_cut_weight.is_finite());
    (best_cut_weight, best_cut)
}

fn test_solve() {
    let data: [(&str, f32, f32, f32); 10] = [
        ("Schwarz", 0.0, 0.5, 0.5),
        (
            "Signalweiß",
            0.961151360371199,
            0.50000000009726,
            0.5000000447823341,
        ),
        (
            "Verkehrsrot",
            0.5346730535533166,
            0.7340950690306774,
            0.6290678476561004,
        ),
        (
            "Lachs",
            0.7882608186750829,
            0.6380500410338976,
            0.5663067939264064,
        ),
        (
            "Zartes Puder",
            0.7582906300295726,
            0.5122111542103134,
            0.5093382344908048,
        ),
        (
            "Luftschloss",
            0.8408536088971593,
            0.42016124595739973,
            0.43023374658457403,
        ),
        (
            "Nilblau",
            0.375673715958732,
            0.47583421184458446,
            0.2646716029139586,
        ),
        (
            "Pflaume",
            0.809408742435091,
            0.5926152589519652,
            0.44128616905899143,
        ),
        (
            "Mittleres Violettrot",
            0.5990910068501093,
            0.7090670259721504,
            0.4296126049311067,
        ),
        (
            "Leuchtendes Rosa",
            0.7158785829249797,
            0.7615687324845466,
            0.45727066011132506,
        ),
    ];
    let points: Vec<Point> = data.map(|(_, r, g, b)| Point::new(vec![r, g, b])).into();
    let metric = Metric {
        norm: crate::typed::Norm::Euclidean,
        invert: false,
    };
    let matrix = Graph::from_points(points, metric).matrix;
    let names = data.map(|(name, ..)| name.to_owned());

    let path = solve_simple(matrix, &names);
    // dbg!(path);
}
