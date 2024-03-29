use super::ImproveContext;
use crate::graph;

pub fn rotate<C: ImproveContext>(ctx: C) -> C::Path {
    let path = ctx.start_path();

    let mut min_cost = f32::INFINITY;
    let mut min_i = 0;
    for i in 0..path.len() {
        let mut inner = path.clone();
        inner.rotate_left(i);
        let cost = ctx.dist_path(inner.iter().copied()).into();
        if cost < min_cost {
            min_cost = cost;
            min_i = i;
            ctx.send_path(path.clone(), Some(i as f32 / path.len() as f32))
        }
    }

    if min_cost < ctx.dist_path(path.iter()).into() {
        let mut inner = path.clone();
        inner.rotate_left(min_i);
        ctx.path_from_indices(inner)
    } else {
        ctx.path_from_indices(path.iter())
    }
}

pub fn two_opt<C: ImproveContext>(ctx: C) -> C::Path {
    fn two_opt_swap(path: &mut graph::Path, v1: usize, v2: usize) {
        let path = path.as_mut();
        path[v1 + 1..v2].reverse();
    }

    let mut improvement = true;
    let mut path = ctx.start_path();
    let mut best_cost = ctx.cost(&path);

    'improvin: while improvement {
        improvement = false;
        for i in 0..path.len() - 1 {
            for j in i + 1..path.len() {
                two_opt_swap(&mut path, i, j);
                let new_cost = ctx.cost(&path);
                if new_cost < best_cost {
                    ctx.send_path(
                        path.iter(),
                        Some((i * path.len() + j) as f32 / ((path.len()) * path.len()) as f32),
                    );
                }
                if new_cost < best_cost {
                    if !ctx.prefer_step() {
                        improvement = true;
                    }
                    best_cost = new_cost;
                    continue 'improvin;
                }
                two_opt_swap(&mut path, i, j);
            }
        }
    }

    ctx.path_from_indices(path.iter())
}

pub fn three_opt<C: ImproveContext>(ctx: C) -> C::Path {
    fn three_opt_swap(path: graph::Path, method: u8, a: usize, b: usize, c: usize) -> graph::Path {
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

    let mut path = ctx.start_path();

    let mut improvement = true;
    let mut best_cost = ctx.cost(&path);
    let mut best_path = path.clone();

    'improvin: while improvement {
        improvement = false;
        let save_path = path.clone();
        for i in 0..path.len() - 4 {
            for j in i + 2..path.len() - 2 {
                for k in j + 2..path.len() {
                    for method in 0..=3 {
                        path = three_opt_swap(path, method, i, j, k);
                        let new_cost = ctx.cost(&path);
                        if new_cost < best_cost || (k == j + 2 && j == i + 2) {
                            ctx.send_path(
                                best_path.iter(),
                                Some(
                                    (i * path.len() + j) as f32
                                        / ((path.len()) * path.len()) as f32,
                                ),
                            );
                        }
                        if new_cost < best_cost {
                            if !ctx.prefer_step() {
                                improvement = true;
                            }
                            best_cost = new_cost;
                            best_path = path.clone();
                            continue 'improvin;
                        } else {
                            save_path.clone_into(&mut path);
                        }
                    }
                }
            }
        }
    }

    ctx.path_from_indices(path.iter())
}

pub fn swap<C: ImproveContext>(ctx: C) -> C::Path {
    let mut improvement = true;
    let mut path = ctx.start_path();
    let mut best_cost = ctx.cost(&path);

    'improvin: while improvement {
        improvement = false;
        for i in 0..path.len() {
            for j in i + 1..path.len() {
                path.as_mut().swap(i, j);
                let new_cost = ctx.cost(&path);
                if new_cost < best_cost {
                    ctx.send_path(
                        path.iter(),
                        Some((i * path.len() + j) as f32 / ((path.len()) * path.len()) as f32),
                    );
                    best_cost = new_cost;
                    if !ctx.prefer_step() {
                        improvement = true;
                    }
                    continue 'improvin;
                } else {
                    path.as_mut().swap(i, j);
                }
            }
        }
    }

    ctx.path_from_indices(path.iter())
}

pub fn inner_rotate<C: ImproveContext>(ctx: C) -> C::Path {
    let mut improvement = true;
    let mut path = ctx.start_path();
    let mut best_cost = ctx.cost(&path);

    'improvin: while improvement {
        improvement = false;
        for start in 0..path.len() {
            ctx.send_path(path.iter(), Some(start as f32 / path.len() as f32));
            for end in start + 1..path.len() {
                for amount in 1..end - start {
                    path.as_mut()[start..end].rotate_left(amount);
                    let new_cost = ctx.cost(&path);
                    if new_cost < best_cost {
                        ctx.send_path(
                            path.iter(),
                            Some(
                                (start * path.len() + end) as f32
                                    / ((path.len() * path.len()) as f32),
                            ),
                        );
                        best_cost = new_cost;
                        if !ctx.prefer_step() {
                            improvement = true;
                        }
                        continue 'improvin;
                    }
                    path.as_mut()[start..end].rotate_right(amount);
                }
            }
        }
    }

    ctx.path_from_indices(path.iter())
}

pub fn simulated_annealing<C: ImproveContext>(ctx: C) -> C::Path {
    let mut path = ctx.start_path();

    let initial_temp: f64 = 0.15;
    let k: f64 = 0.00000000025;
    let mut temperature = initial_temp;
    let mut i = 0;
    let mut cost = ctx.cost(&path);
    let mut path_approx = path.clone();
    let mut path_approx_cost = cost;

    while temperature > 0.000000005 {
        if i % (1 << 24) == 0 {
            ctx.send_path(
                path_approx.iter(),
                Some(1.0 - (temperature / initial_temp) as f32),
            );
        }
        let index1 = fastrand::usize(..path.len());
        let index2 = fastrand::usize(..path.len());

        path.swap(index1, index2);
        let new_cost = ctx.cost(&path);
        let cost_delta = new_cost - cost;
        // let cost_delta = path.cost_delta_under_swap(index1, index2, norm);
        let thresh = f32::exp(-(cost_delta / temperature as f32));
        if cost_delta < 0.0 || fastrand::f32() < thresh {
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

    ctx.path_from_indices(path_approx.iter())
}
