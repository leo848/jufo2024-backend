use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use simple_websockets::{Event, EventHub, Message, Responder};

use crate::{
    action::{IntegerSortContext, DistPathCreateContext, DistPathImproveContext},
    autorestart,
    dist_graph::Path,
    error::Error,
    integer_sort,
    dist_path::{self, creation::PathCreation, improvement::PathImprovement},
};

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum IntegerSortAlgorithm {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
}

impl IntegerSortAlgorithm {
    pub fn implementation(self) -> fn(IntegerSortContext) -> Vec<u64> {
        match self {
            Self::Bubble => integer_sort::bubble,
            Self::Insertion => integer_sort::insertion,
            Self::Selection => integer_sort::selection,
            Self::Merge => integer_sort::merge,
            Self::Quick => integer_sort::quick,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PathCreateMethod {
    Transmute,
    Random,
    NearestNeighbor,
    BruteForce,
    Greedy,
    Christofides,
}

impl PathCreateMethod {
    #[inline]
    pub fn implementation(self) -> fn(DistPathCreateContext) -> Path {
        match self {
            Self::Transmute => dist_path::create::transmute,
            Self::Random => dist_path::create::random,
            Self::NearestNeighbor => dist_path::create::nearest_neighbor,
            Self::BruteForce => dist_path::create::brute_force,
            Self::Greedy => dist_path::create::greedy,
            Self::Christofides => dist_path::create::christofides,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PathImproveMethod {
    Rotate,
    Swap,
    TwoOpt,
    ThreeOpt,
    SimulatedAnnealing,
}

impl PathImproveMethod {
    #[inline]
    pub fn implementation(self) -> fn(DistPathImproveContext) -> Path {
        match self {
            Self::Rotate => dist_path::improve::rotate,
            Self::TwoOpt => dist_path::improve::two_opt,
            Self::ThreeOpt => dist_path::improve::three_opt,
            Self::Swap => dist_path::improve::swap,
            Self::SimulatedAnnealing => dist_path::improve::simulated_annealing,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub enum Norm {
    Manhattan,
    #[default]
    Euclidean,
    Max,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    SortNumbers {
        numbers: Vec<u64>,
        algorithm: IntegerSortAlgorithm,
    },
    CreatePath {
        dimensions: u8,
        values: Vec<Vec<f32>>,
        #[serde(default)]
        norm: Norm,
        method: PathCreateMethod,
    },
    ImprovePath {
        dimensions: u8,
        path: Vec<Vec<f32>>,
        #[serde(default)]
        norm: Norm,
        method: PathImproveMethod,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    Log {
        message: String,
    },
    Action {
        action: Action,
        #[serde(default)]
        latency: u64,
    },
    Latency,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Highlight {
    Compare,
    Swap,
    Correct,
    Consider,
    Smaller,
    Larger,
    Pivot,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Output {
    Log {
        message: String,
    },
    Error {
        error: Error,
    },
    SortedNumbers {
        done: bool,
        numbers: Vec<u64>,
        highlight: Vec<(usize, Highlight)>,
    },
    PathCreation(PathCreation),
    PathImprovement(PathImprovement),
    #[serde(rename_all = "camelCase")]
    Latency {
        time_millis: u128,
    },
}

/// Polls the event hub for a new event.
/// On join, sends a welcome message and adds client to a list.
/// On message, parses the message from JSON using Serde and returns the result.
/// On leave, removes the client from the list.
pub fn poll(event_hub: &EventHub, clients: &mut HashMap<u64, Responder>) -> (Responder, Input) {
    autorestart::update();
    loop {
        match event_hub.poll_event() {
            Event::Connect(id, responder) => {
                println!("Connect: #{id}");
                send(
                    &responder,
                    Output::Log {
                        message: format!("WebSocket successfully connected. You are client #{id}."),
                    },
                );
                clients.insert(id, responder);
            }
            Event::Disconnect(id) => {
                println!("Disconnect: #{id}");
                clients.remove(&id);
            }
            Event::Message(id, message) => {
                let client = &clients[&id];
                match message {
                    Message::Binary(_) => {
                        error(client, Error::BinaryData);
                    }
                    Message::Text(string) => {
                        // eprintln!("recv {}", string);
                        let result = serde_json::from_str(&string);
                        let result: Input = match result {
                            Ok(result) => result,
                            Err(e) => {
                                eprintln!("Invalid input: {string}");
                                error(
                                    client,
                                    Error::Serde {
                                        original: string,
                                        error: e.to_string(),
                                    },
                                );
                                continue;
                            }
                        };
                        return (client.clone(), result);
                    }
                }
            }
        }
    }
}

pub fn send(client: &Responder, message: impl Into<Output>) {
    autorestart::update();
    let string = serde_json::to_string(&message.into()).expect("Bug: could not serialize string");
    // eprintln!("send {}", string);
    client.send(Message::Text(string));
}

pub fn error(client: &Responder, error: Error) {
    send(client, Output::Error { error });
}
