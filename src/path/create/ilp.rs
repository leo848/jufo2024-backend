use crate::{
    action::{ActionContext, DistPathCreateContext},
    dist_graph::Point,
    graph::{Graph, Matrix, Path},
    path::CreateContext,
    typed::Metric,
};

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    let matrix: Matrix = ctx.adjacency_matrix();

    dbg!(matrix);

    todo!()
}

pub fn solve_simple(matrix: Matrix) -> Path {
    dbg!(matrix);

    todo!();
}

#[test]
fn test_solve() {
    let points: Vec<Point> = [
        [0.0, 0.5, 0.5],
        [0.961151360371199, 0.50000000009726, 0.5000000447823341],
        [0.5346730535533166, 0.7340950690306774, 0.6290678476561004],
        [0.7882608186750829, 0.6380500410338976, 0.5663067939264064],
        [0.7582906300295726, 0.5122111542103134, 0.5093382344908048],
        [0.8408536088971593, 0.42016124595739973, 0.43023374658457403],
        [0.375673715958732, 0.47583421184458446, 0.2646716029139586],
        [0.809408742435091, 0.5926152589519652, 0.44128616905899143],
        [0.5990910068501093, 0.7090670259721504, 0.4296126049311067],
        [0.7158785829249797, 0.7615687324845466, 0.45727066011132506],
    ]
    .map(|[r, g, b]| Point::new(vec![r, g, b]))
    .into();
    let metric = Metric {
        norm: crate::typed::Norm::Euclidean,
        invert: false,
    };
    let matrix = Graph::from_points(points, metric).matrix;

    let path = solve_simple(matrix);
    dbg!(path);
}
