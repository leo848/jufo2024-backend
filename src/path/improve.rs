use simple_websockets::Responder;

use crate::{
    graph::{Edge, Path},
    path::improvement::PathImprovement,
    typed::send,
};

pub fn rotate(_client: &Responder, dim: u8, old_path: Path) -> Path {
    let edges = old_path.clone().into_edges();
    let (max_idx, max_edge) = edges
        .into_iter()
        .enumerate()
        .max_by_key(|(_, edge)| edge.dist())
        .expect("should be nonempty");
    if max_edge.dist() > Edge::new(old_path[0].clone(), old_path[old_path.len() - 1].clone()).dist()
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

pub fn two_opt(client: &Responder, _dim: u8, old_path: Path) -> Path {
    fn two_opt_swap(path: &mut Path, v1: usize, v2: usize) {
        let path = path.as_mut();
        path[v1 + 1..v2].reverse();
    }

    let mut path = old_path.clone();

    let mut improvement = true;
    let mut best_cost = path.cost();

    'improvin: while improvement {
        improvement = false;
        for i in 0..path.len() - 1 {
            for j in i + 1..path.len() {
                two_opt_swap(&mut path, i, j);
                let new_cost = path.cost();
                if new_cost < best_cost {
                    improvement = true;
                    best_cost = new_cost;
                    send(client, PathImprovement::from_path(path.clone()));
                    continue 'improvin;
                }
                two_opt_swap(&mut path, i, j);
            }
        }
    }

    path
}

pub fn three_opt(client: &Responder, _dim: u8, old_path: Path) -> Path {
    fn three_opt_swap(path: &mut Path, method: u8, a: usize, b: usize, c: usize) {
        let path = path.as_mut();
        match method {
            0 => {
                path[c..b-1].reverse();
            }
            _ => panic!("Wrong method"),
        }
    }

    let mut path = old_path.clone();

    let mut improvement = true;
    let mut best_cost = path.cost();

    'improvin: while improvement {
        improvement = false;
        let save_path = path.clone();
        for i in 0..path.len() - 2 {
            for j in i + 1 .. path.len() - 1 {
                for k in j + 1 .. path.len() {
                    for method in 0..1 {
                        three_opt_swap(&mut path, method, i, j, k);
                        let new_cost = path.cost();
                        if new_cost < best_cost {
                            improvement = true;
                            best_cost = new_cost;
                            send(client, PathImprovement::from_path(path.clone()));
                            continue 'improvin;
                        } else {
                            save_path.clone_into(&mut path);
                        }
                    }
                }
            }
        }
    }

    todo!()
}
