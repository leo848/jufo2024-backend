use crate::graph::Path;
use crate::path::{CreateContext, Matrix};

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
