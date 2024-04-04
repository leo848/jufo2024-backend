use crate::dist_graph::Point;
use crate::CreateContext;
use crate::Graph;
use crate::{graph::Path, path::Matrix, typed::Metric};
use coin_cbc::Col;
use coin_cbc::{Model, Sense};
use itertools::Itertools;

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    let matrix = ctx.adjacency_matrix();

    let size = matrix.dim();
    let node_indices = { || (0..size) };

    let mut model = Model::default();

    let variables = {
        node_indices()
            .map(|i| node_indices().map(|j| model.add_binary()).collect_vec())
            .collect_vec()
    };

    // (matrix[(i, j)].into(), (0.0, 1.0));

    todo!()
}

pub fn solve_simple(matrix: Matrix, names: &[String]) -> Path {
    let size = matrix.dim();
    let node_indices = { || (0..size) };

    let mut model = Model::default();
    model.set_obj_sense(Sense::Minimize);

    let x: Vec<Vec<Col>> = {
        node_indices()
            .map(|i| node_indices().map(|j| model.add_binary()).collect_vec())
            .collect_vec()
    };

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

    let sol = model.solve();
    for i in node_indices() {
        for j in node_indices() {
            let value = sol.col(x[i][j]);
            if value == 1.0 {
                println!("{i} -> {j}");
            }
        }
    }
    dbg!(model.to_raw().obj_value());

    // (matrix[(i, j)].into(), (0.0, 1.0));

    todo!()
}

#[test]
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
