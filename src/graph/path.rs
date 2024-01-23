use core::{
    ops::{Add, Index},
    slice::SliceIndex,
};

use derive_more::Constructor;
use itertools::Itertools;
use serde::Serialize;

use super::Edge;

#[derive(Constructor, Debug, Clone, Serialize)]
pub struct Path(Vec<usize>);

#[allow(dead_code)]
impl Path {
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().copied()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn into_inner(self) -> Vec<usize> {
        self.0
    }

    pub fn into_edges(self) -> Vec<Edge> {
        self.iter().tuple_windows().map(|(l, r)| Edge::new(l, r)).collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, node_index: usize) {
        self.0.push(node_index);
    }

    pub fn into_slice(self, range: impl SliceIndex<[usize], Output = [usize]>) -> Path {
        Path(self.0[range].to_vec())
    }

    pub fn slice(&self, range: impl SliceIndex<[usize], Output = [usize]>) -> Path {
        self.clone().into_slice(range)
    }

    pub fn rev(mut self) -> Path {
        self.0.reverse();
        self
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.0.get(index).map(|&i| i)
    }
}

impl Add<&Path> for Path {
    type Output = Path;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.0.extend_from_slice(&rhs.0);
        self
    }
}

impl Add for Path {
    type Output = Path;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl<Output: ?Sized, Idx: SliceIndex<[usize], Output = Output>> Index<Idx> for Path {
    type Output = <Idx as SliceIndex<[usize]>>::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}
