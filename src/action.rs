use std::{thread, time::Duration};

use simple_websockets::Responder;

use crate::{
    typed::{send, Norm},
    Output, Path, Points,
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
pub struct PathCreateContext {
    pub action: ActionContext,
    pub dim: u8,
    pub points: Points,
    pub norm: Norm,
}

#[derive(Clone)]
pub struct PathImproveContext {
    pub action: ActionContext,
    pub dim: u8,
    pub path: Path,
    pub norm: Norm,
}
