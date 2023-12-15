use simple_websockets::Responder;

use crate::{Path, Points};

pub struct ActionContext {
    pub client: Responder,
    pub latency: u64,
}

pub struct IntegerContext {
    pub action: ActionContext,
    pub numbers: Vec<u64>,
}

pub struct PathCreateContext {
    pub action: ActionContext,
    pub dim: u8,
    pub points: Points,
}

pub struct PathImproveContext {
    pub action: ActionContext,
    pub dim: u8,
    pub path: Path,
}
