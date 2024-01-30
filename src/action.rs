use std::{thread, time::Duration};

use simple_websockets::Responder;

use crate::{
    dist_graph,
    graph::{self, Graph},
    typed::{send, Norm},
    Output,
};

#[derive(Clone)]
pub struct ActionContext {
    pub client: Responder,
    pub latency: u64,
}

impl ActionContext {
    pub fn send(&self, message: impl Into<Output>) {
        send(&self.client, message);
        thread::sleep(Duration::from_millis(self.latency));
    }
}

#[derive(Clone)]
pub struct IntegerSortContext {
    pub action: ActionContext,
    pub numbers: Vec<u64>,
}

#[derive(Clone)]
pub struct DistPathCreateContext {
    pub action: ActionContext,
    pub dim: u8,
    pub points: Vec<dist_graph::Point>,
    pub graph: Graph,
    pub norm: Norm,
}

#[derive(Clone)]
pub struct DistPathImproveContext {
    pub action: ActionContext,
    pub dim: u8,
    pub path: dist_graph::Path,
    pub graph: Graph,
    pub norm: Norm,
}

#[derive(Clone)]
pub struct PathCreateContext {
    pub action: ActionContext,
    pub graph: graph::Graph,
}

#[derive(Clone)]
pub struct PathImproveContext {
    pub action: ActionContext,
    pub path: graph::Path,
    pub graph: graph::Graph,
}
