use simple_websockets::Responder;

use crate::graph::{Path, Edge};

pub fn rotate(_client: &Responder, dim: u8, old_path: Path) -> Path {
    let edges = old_path.clone().to_edges();
    let (max_idx, max_edge) = edges.into_iter().enumerate().max_by(|(_,e1),(_,e2)|e1.dist().total_cmp(&e2.dist())).expect("should be nonempty");
    if max_edge.dist() > Edge::new(old_path[0].clone(), old_path[old_path.len()-1].clone()).dist() {
        let mut new_path = old_path.into_inner();
        let len = new_path.len();
        // So rotieren, dass min_idx auf -1 liegt.
        let left = (max_idx +1) % len;
        new_path.rotate_left(left);
        Path::try_new(new_path, dim).unwrap()
    } else {
        old_path
    }
}

pub fn two_opt(client: &Responder, dim: u8, old_path: Path) -> Path {
    todo!();
}
