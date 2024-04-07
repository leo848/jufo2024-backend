pub mod create;
pub mod creation;
pub mod improve;
pub mod improvement;

use std::ops::Range;

use itertools::Itertools;

use crate::{
    action::{DistPathCreateContext, DistPathImproveContext, PathCreateContext},
    dist_graph,
    graph::{self, Graph, Matrix},
    path::{creation::PathCreation, improvement::PathImprovement},
    pool::OptionsPool,
    DistPathCreation, DistPathImprovement, PathImproveContext,
};

const PESSIMAL: bool = false;

pub trait PathContext {
    type Path;

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path;
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
    fn adjacency_matrix(&self) -> Matrix {
        Matrix::new(
            self.node_indices()
                .map(|i| self.node_indices().map(|j| self.dist(i, j)).collect())
                .collect(),
        )
        .expect("node_indices misbehaved")
    }
    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>);

    fn options(&self) -> &OptionsPool;
}

pub trait CreateContext: PathContext + Clone {
    fn send_edges(&self, path: impl IntoIterator<Item = (usize, usize)>, progress: Option<f32>);
    fn rotate_left(self, index: usize) -> Self
    where
        Self: Sized;
}

pub trait ImproveContext: PathContext {
    fn start_path(&self) -> graph::Path {
        graph::Path::new(self.node_indices().collect())
    }
    fn send_path_for_reactivity(
        &self,
        path: impl IntoIterator<Item = usize>,
        progress: Option<f32>,
    );
    fn prefer_step(&self) -> bool;
}

impl PathContext for DistPathCreateContext {
    type Path = dist_graph::Path;

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        dist_graph::Path::try_new(
            path.into_iter()
                .map(|idx| self.points[idx].clone())
                .collect(),
            self.dim,
        )
        .expect("invalid dimension")
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        let value: f32 = self.graph.weight(nindex1, nindex2).into();
        if PESSIMAL {
            -value
        } else {
            value
        }
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut dpc = DistPathCreation::from_path(self.path_from_indices(path));
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }

    fn options(&self) -> &OptionsPool {
        &self.action.pool
    }
}

impl CreateContext for DistPathCreateContext {
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

    fn rotate_left(self, index: usize) -> Self
    where
        Self: Sized,
    {
        let Self {
            action,
            dim,
            mut points,
            graph: _,
            metric,
        } = self;
        points.rotate_left(index);
        Self {
            action,
            dim,
            points: points.clone(),
            graph: Graph::from_points(points, metric),
            metric,
        }
    }
}

impl PathContext for PathCreateContext {
    type Path = graph::Path;

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        graph::Path::new(path.into_iter().collect())
    }

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        let value: f32 = self.graph.weight(nindex1, nindex2).into();
        if PESSIMAL {
            -value
        } else {
            value
        }
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut pc = PathCreation::from_path(graph::Path::new(path.into_iter().collect_vec()));
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }

    fn options(&self) -> &OptionsPool {
        &self.action.pool
    }
}

impl CreateContext for PathCreateContext {
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

    fn rotate_left(self, index: usize) -> Self
    where
        Self: Sized,
    {
        let Self { action, graph } = self;
        Self {
            action,
            graph: graph.rotate_left(index),
        }
    }
}

impl PathContext for DistPathImproveContext {
    type Path = dist_graph::Path;

    fn len(&self) -> usize {
        self.path.len()
    }

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        dist_graph::Path::try_new(
            path.into_iter().map(|idx| self.path[idx].clone()).collect(),
            self.dim,
        )
        .expect("invalid dimension")
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        let value = self.graph.weight(nindex1, nindex2).into_inner();
        if PESSIMAL {
            -value
        } else {
            value
        }
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut dpc = DistPathImprovement::from_path(self.path_from_indices(path));
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }

    fn options(&self) -> &OptionsPool {
        &self.action.pool
    }
}

impl ImproveContext for DistPathImproveContext {
    fn send_path_for_reactivity(
        &self,
        path: impl IntoIterator<Item = usize>,
        progress: Option<f32>,
    ) {
        let mut dpc = DistPathImprovement::from_path(self.path_from_indices(path)).not_better();
        if let Some(p) = progress {
            dpc = dpc.progress(p)
        }
        self.action.send(dpc);
    }

    fn prefer_step(&self) -> bool {
        self.prefer_step
    }
}

impl PathContext for PathImproveContext {
    type Path = graph::Path;

    fn path_from_indices(&self, path: impl IntoIterator<Item = usize>) -> Self::Path {
        graph::Path::new(path.into_iter().collect())
    }

    fn len(&self) -> usize {
        self.graph.size()
    }

    fn dist(&self, nindex1: usize, nindex2: usize) -> f32 {
        let value = self.graph.weight(nindex1, nindex2).into_inner();
        if PESSIMAL {
            -value
        } else {
            value
        }
    }

    fn send_path(&self, path: impl IntoIterator<Item = usize>, progress: Option<f32>) {
        let mut pc = PathImprovement::from_path(graph::Path::new(path.into_iter().collect_vec()));
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }

    fn options(&self) -> &OptionsPool {
        &self.action.pool
    }
}

impl ImproveContext for PathImproveContext {
    fn send_path_for_reactivity(
        &self,
        path: impl IntoIterator<Item = usize>,
        progress: Option<f32>,
    ) {
        let mut pc = PathImprovement::from_path(graph::Path::new(path.into_iter().collect_vec()))
            .not_better();
        if let Some(p) = progress {
            pc = pc.progress(p);
        }
        self.action.send(pc);
    }

    fn prefer_step(&self) -> bool {
        self.prefer_step
    }

    fn start_path(&self) -> graph::Path {
        self.path.clone()
    }
}
