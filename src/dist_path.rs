use std::ops::Range;

use itertools::Itertools;

use crate::{
    action::{DistPathCreateContext, DistPathImproveContext, PathCreateContext},
    dist_graph::{self, Cost},
    graph::{self, Weight},
    path::creation::PathCreation,
    DistPathCreation, DistPathImprovement, PathImproveContext,
};

pub mod create;
pub mod creation;
pub mod improve;
pub mod improvement;

pub trait CreateContext {
    type Path;
    type Distance: Eq + Ord + Into<f32>;
    fn len(&self) -> usize;
    fn node_indices(&self) -> Range<usize> {
        0..self.len()
    }
    fn dist(&self, nindex1: usize, nindex2: usize) -> Self::Distance;
    fn dist_path(&self, path: &Self::Path) -> Self::Distance;
    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>);
    fn send_edges(&self, path: impl IntoIterator<Item = (usize, usize)>, progress: Option<f32>);
    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path;
}

impl CreateContext for DistPathCreateContext {
    type Path = dist_graph::Path;
    type Distance = Cost;

    fn len(&self) -> usize {
        self.points.len()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> Cost {
        self.points[nindex1].dist(&self.points[nindex2], self.norm)
    }

    fn dist_path(&self, path: &Self::Path) -> Self::Distance {
        path.cost(self.norm)
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        dist_graph::Path::try_new(
            path.into_iter()
                .map(|idx| self.points[idx].clone())
                .collect(),
            self.dim,
        )
        .expect("invalid dimension")
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut dpc = DistPathCreation::from_path(self.path_from_indices(path));
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }

    fn send_edges(&self, edges: impl IntoIterator<Item = (usize, usize)>, progress: Option<f32>) {
        let mut dpc = DistPathCreation::from_edges(
            edges
                .into_iter()
                .map(|(f, t)| dist_graph::Edge::new(self.points[f].clone(), self.points[t].clone()))
                .collect(),
        );
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }
}

impl CreateContext for PathCreateContext {
    type Path = graph::Path;
    type Distance = Weight;

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> Weight {
        self.graph.weight(nindex1, nindex2)
    }

    fn dist_path(&self, path: &Self::Path) -> Self::Distance {
        self.graph.path_weight(path)
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        graph::Path::new(path.into_iter().collect())
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut pc = PathCreation::from_path(graph::Path::new(path.into_iter().collect_vec()));
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }

    fn send_edges(&self, path: impl IntoIterator<Item = (usize, usize)>, progress: Option<f32>) {
        let mut pc = PathCreation::from_edges(
            path.into_iter()
                .map(|(l, r)| graph::Edge::new(l, r))
                .collect(),
        );
        if let Some(p) = progress {
            pc = pc.progress(p)
        }
        self.action.send(pc);
    }
}

pub trait ImproveContext {
    type Path;
    type Distance: Eq + Ord + Into<f32>;
    fn len(&self) -> usize;
    fn node_indices(&self) -> Range<usize> {
        0..self.len()
    }
    fn start_path(&self) -> graph::Path {
        graph::Path::new(self.node_indices().collect())
    }
    fn dist(&self, nindex1: usize, nindex2: usize) -> Self::Distance;
    fn dist_path(&self, path: &Self::Path) -> Self::Distance;
    fn cost(&self, path: &graph::Path) -> f32 {
        self.dist_path(&self.path_from_indices(path.iter())).into()
    }
    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>);
    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path;
}

impl ImproveContext for DistPathImproveContext {
    type Path = dist_graph::Path;
    type Distance = Cost;

    fn len(&self) -> usize {
        self.path.len()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> Cost {
        self.path[nindex1].dist(&self.path[nindex2], self.norm)
    }

    fn dist_path(&self, path: &Self::Path) -> Self::Distance {
        path.cost(self.norm)
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut dpc = DistPathImprovement::from_path(self.path_from_indices(path));
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        dist_graph::Path::try_new(
            path.into_iter().map(|idx| self.path[idx].clone()).collect(),
            self.dim,
        )
        .expect("invalid dimension")
    }
}

impl ImproveContext for PathImproveContext {
    type Path = graph::Path;
    type Distance = Weight;

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> Weight {
        self.graph.weight(nindex1, nindex2)
    }

    fn dist_path(&self, path: &Self::Path) -> Self::Distance {
        self.graph.path_weight(path)
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        graph::Path::new(path.into_iter().collect())
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut pc = PathCreation::from_path(graph::Path::new(path.into_iter().collect_vec()));
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }
}
