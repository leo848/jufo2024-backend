use std::{thread, time::Duration};

use simple_websockets::Responder;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    dist_graph,
    graph::{self, Graph},
    typed::{send, Norm},
    Output,
};

static LAST_ACTION_SEND: Mutex<u128> = Mutex::new(0);

#[derive(Clone)]
pub struct ActionContext {
    pub client: Responder,
    pub latency: u64,
}

impl ActionContext {
    pub fn send(&self, message: impl Into<Output>) {
        let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("Zeit rückwärts").as_millis();
        let mut last_send_time = LAST_ACTION_SEND.lock().unwrap();
        let duration_millis = (self.latency as u128).checked_sub(current - *last_send_time);
        if let Some(sleep_time) = duration_millis {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("Zeit rückwärts").as_millis();
        *last_send_time = current;
        send(&self.client, message);
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
