use std::{thread, time::Duration};

use simple_websockets::Responder;

use crate::{typed::send, Output, Path, Points};

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
