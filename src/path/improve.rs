use crate::{
    action::PathImproveContext,
    graph::{Edge, Path},
    path::improvement::PathImprovement,
};

pub fn rotate(ctx: PathImproveContext) -> Path {
    let PathImproveContext {
        action,
        path: old_path,
        dim,
        norm,
    } = ctx;

    for i in 0..old_path.len() {
        let mut inner = old_path.clone().into_inner();
        inner.rotate_left(i);
        let rotated = Path::try_new(inner, dim).unwrap();
        action.send(PathImprovement::from_path(rotated).better(false));
    }

    let edges = old_path.clone().into_edges();
    let (max_idx, max_edge) = edges
        .into_iter()
        .enumerate()
        .max_by_key(|(_, edge)| edge.dist(norm))
        .expect("should be nonempty");
    if max_edge.dist(norm)
        > Edge::new(old_path[0].clone(), old_path[old_path.len() - 1].clone()).dist(norm)
    {
        let mut new_path = old_path.into_inner();
        let len = new_path.len();
        // So rotieren, dass min_idx auf -1 liegt.
        let left = (max_idx + 1) % len;
        new_path.rotate_left(left);
        Path::try_new(new_path, dim).unwrap()
    } else {
        old_path
    }
}

pub fn two_opt(ctx: PathImproveContext) -> Path {
    fn two_opt_swap(path: &mut Path, v1: usize, v2: usize) {
        let path = path.as_mut();
        path[v1 + 1..v2].reverse();
    }

    let PathImproveContext {
        action,
        mut path,
        dim: _,
        norm,
    } = ctx;

    let mut improvement = true;
    let mut best_cost = path.cost(norm);

    'improvin: while improvement {
        improvement = false;
        for i in 0..path.len() - 1 {
            for j in i + 1..path.len() {
                two_opt_swap(&mut path, i, j);
                let new_cost = path.cost(norm);
                if new_cost < best_cost {
                    action.send(PathImprovement::from_path(path.clone()).progress(
                        (i * path.len() + j) as f32 / ((path.len()) * path.len()) as f32,
                    ));
                }
                if new_cost < best_cost {
                    improvement = true;
                    best_cost = new_cost;
                    continue 'improvin;
                }
                two_opt_swap(&mut path, i, j);
            }
        }
    }

    path
}

pub fn three_opt(ctx: PathImproveContext) -> Path {
    fn three_opt_swap(path: Path, method: u8, a: usize, b: usize, c: usize) -> Path {
        let [a, c, e] = [a, b, c];
        let [b, d, f] = [a + 1, b + 1, c + 1];
        match method {
            0 => {
                path.slice(..=a)
                    + path.slice(b..=c).rev()
                    + path.slice(d..=e).rev()
                    + path.slice(f..)
            }
            1 => path.slice(..=a) + path.slice(d..=e) + path.slice(b..=c) + path.slice(f..),
            2 => path.slice(..=a) + path.slice(d..=e) + path.slice(b..=c).rev() + path.slice(f..),
            3 => path.slice(..=a) + path.slice(d..=e).rev() + path.slice(b..=c) + path.slice(f..),
            _ => panic!("Wrong method"),
        }
    }

    let PathImproveContext {
        action,
        dim: _,
        mut path,
        norm,
    } = ctx;

    let mut improvement = true;
    let mut best_cost = path.cost(norm);

    'improvin: while improvement {
        improvement = false;
        let save_path = path.clone();
        for i in 0..path.len() - 4 {
            for j in i + 2..path.len() - 2 {
                for k in j + 2..path.len() {
                    for method in 0..=3 {
                        path = three_opt_swap(path, method, i, j, k);
                        let new_cost = path.cost(norm);
                        if new_cost < best_cost || (k == j + 2 && j == i + 2) {
                            action.send(
                                PathImprovement::from_path(path.clone())
                                    .progress(
                                        (i * path.len() + j) as f32
                                            / ((path.len()) * path.len()) as f32,
                                    )
                                    .better(new_cost < best_cost),
                            );
                        }
                        if new_cost < best_cost {
                            improvement = true;
                            best_cost = new_cost;
                            continue 'improvin;
                        } else {
                            save_path.clone_into(&mut path);
                        }
                    }
                }
            }
        }
    }

    path
}

pub fn swap(ctx: PathImproveContext) -> Path {
    let PathImproveContext {
        action,
        mut path,
        dim: _,
        norm,
    } = ctx;

    let mut improvement = true;
    let mut best_cost = path.cost(norm);

    'improvin: while improvement {
        improvement = false;
        for i in 0..path.len() {
            for j in i + 1..path.len() {
                path.as_mut().swap(i, j);
                let new_cost = path.cost(norm);
                if new_cost < best_cost {
                    action.send(PathImprovement::from_path(path.clone()).progress(
                        (i * path.len() + j) as f32 / ((path.len()) * path.len()) as f32,
                    ));
                    best_cost = new_cost;
                    improvement = true;
                    continue 'improvin;
                } else {
                    path.as_mut().swap(i, j);
                }
            }
        }
    }

    path
}

pub fn simulated_annealing(ctx: PathImproveContext) -> Path {
    let PathImproveContext {
        action,
        mut path,
        dim: _,
        norm,
    } = ctx;

    let initial_temp: f64 = 0.15;
    let k: f64 = 0.00000000025;
    let mut temperature = initial_temp;
    let mut i = 0;
    let mut cost = path.cost(norm);
    let mut path_approx = path.clone();
    let mut path_approx_cost = cost;

    while temperature > 0.000000005 {
        if i % (1 << 24) == 0 {
            action.send(
                PathImprovement::from_path(path_approx.clone())
                    .progress(1.0 - (temperature / initial_temp) as f32),
            );
        }
        let index1 = fastrand::usize(..path.len());
        let index2 = fastrand::usize(..path.len());

        path.swap(index1, index2);
        let new_cost = path.cost(norm);
        let cost_delta = new_cost.into_inner() - cost.into_inner();
        // let cost_delta = path.cost_delta_under_swap(index1, index2, norm);
        if cost_delta < 0.0 || fastrand::f32() < f32::exp(-((cost_delta) / temperature as f32)) {
            // path.swap(index1, index2);
            // cost += cost_delta;
            cost = new_cost;
        } else {
            path.swap(index1, index2);
        }
        if new_cost < path_approx_cost {
            path.clone_into(&mut path_approx);
            path_approx_cost = new_cost;
        }
        temperature -= k;
        i += 1;
    }

    path_approx
}
