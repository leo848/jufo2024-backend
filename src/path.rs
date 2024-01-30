pub mod create;
pub mod creation;
pub mod improve;
pub mod improvement;

use std::ops::Range;

use itertools::Itertools;

use crate::{
    action::{DistPathCreateContext, DistPathImproveContext, PathCreateContext},
    dist_graph, graph,
    path::{creation::PathCreation, improvement::PathImprovement},
    DistPathCreation, DistPathImprovement, PathImproveContext,
};

pub trait CreateContext {
    type Path;
    fn len(&self) -> usize;
    fn node_indices(&self) -> Range<usize> {
        0..self.len()
    }
    fn dist(&self, nindex1: usize, nindex2: usize) -> f32;
    fn dist_path(&self, path: impl IntoIterator<Item = usize>) -> f32 {
        path.into_iter()
            .tuple_windows()
            .map(|(l, r)| self.dist(l, r))
            .sum()
    }
    fn cost(&self, path: &graph::Path) -> f32 {
        self.dist_path(path.iter())
    }
    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>);
    fn send_edges(&self, path: impl IntoIterator<Item = (usize, usize)>, progress: Option<f32>);
    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path;
}

impl CreateContext for DistPathCreateContext {
    type Path = dist_graph::Path;

    fn len(&self) -> usize {
        self.points.len()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        self.graph.weight(nindex1, nindex2).into()
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

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        self.graph.weight(nindex1, nindex2).into()
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
    fn len(&self) -> usize;
    fn node_indices(&self) -> Range<usize> {
        0..self.len()
    }
    fn start_path(&self) -> graph::Path {
        graph::Path::new(self.node_indices().collect())
    }
    fn dist(&self, nindex1: usize, nindex2: usize) -> f32;
    fn dist_path(&self, path: impl IntoIterator<Item = usize>) -> f32 {
        path.into_iter()
            .tuple_windows()
            .map(|(l, r)| self.dist(l, r))
            .sum()
    }
    fn cost(&self, path: &graph::Path) -> f32 {
        self.dist_path(path.iter()).into()
    }
    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>);
    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path;
}

impl ImproveContext for DistPathImproveContext {
    type Path = dist_graph::Path;

    fn len(&self) -> usize {
        self.path.len()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        self.graph.weight(nindex1, nindex2).into_inner()
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

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        self.graph.weight(nindex1, nindex2).into_inner()
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        graph::Path::new(path.into_iter().collect())
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut pc = PathImprovement::from_path(graph::Path::new(path.into_iter().collect_vec()));
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }
}
