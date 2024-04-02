use crate::dist_graph::Point;
use crate::graph::Path;
use crate::path::{CreateContext, Matrix};
use crate::typed::Metric;
use crate::Graph;

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    let matrix: Matrix = ctx.adjacency_matrix();

    dbg!(matrix);

    todo!()
}

#[cfg(test)]
#[allow(unused)]
fn solve_simple(matrix: Matrix, names: &[String]) -> Path {
    todo!();
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
