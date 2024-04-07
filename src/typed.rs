use crate::pool::OptionsPool;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use simple_websockets::{Event, EventHub, Message, Responder};

use crate::{
    action::{
        DistPathCreateContext, DistPathImproveContext, IntegerSortContext, PathCreateContext,
        PathImproveContext,
    },
    autorestart, dist_graph,
    dist_path::{creation::DistPathCreation, improvement::DistPathImprovement},
    error::Error,
    graph, integer_sort, path,
    path::{creation::PathCreation, improvement::PathImprovement},
    DEBUG_WS,
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
    pub fn implementation(self) -> fn(IntegerSortContext) -> Vec<i64> {
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
    OptimalNearestNeighbor,
    BruteForce,
    Greedy,
    HeldKarp,
    Ilp,
    Insertion,
}

impl PathCreateMethod {
    #[inline]
    pub fn dist_implementation(self) -> fn(DistPathCreateContext) -> dist_graph::Path {
        match self {
            Self::Transmute => path::create::transmute,
            Self::Random => path::create::random,
            Self::NearestNeighbor => path::create::nearest_neighbor,
            Self::OptimalNearestNeighbor => path::create::optimal_nearest_neighbor,
            Self::BruteForce => path::create::brute_force,
            Self::Greedy => path::create::greedy,
            Self::Ilp => path::create::ilp,
            Self::HeldKarp => path::create::held_karp,
            Self::Insertion => path::create::insertion,
        }
    }

    #[inline]
    pub fn implementation(self) -> fn(PathCreateContext) -> graph::Path {
        match self {
            Self::Transmute => path::create::transmute,
            Self::Random => path::create::random,
            Self::NearestNeighbor => path::create::nearest_neighbor,
            Self::OptimalNearestNeighbor => path::create::optimal_nearest_neighbor,
            Self::BruteForce => path::create::brute_force,
            Self::Greedy => path::create::greedy,
            Self::Ilp => path::create::ilp,
            Self::HeldKarp => path::create::held_karp,
            Self::Insertion => path::create::insertion,
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
    InnerRotate,
    SimulatedAnnealing,
}

impl PathImproveMethod {
    #[inline]
    pub fn dist_implementation(self) -> fn(DistPathImproveContext) -> dist_graph::Path {
        match self {
            Self::Rotate => path::improve::rotate,
            Self::InnerRotate => path::improve::inner_rotate,
            Self::TwoOpt => path::improve::two_opt,
            Self::ThreeOpt => path::improve::three_opt,
            Self::Swap => path::improve::swap,
            Self::SimulatedAnnealing => path::improve::simulated_annealing,
        }
    }

    #[inline]
    pub fn implementation(self) -> fn(PathImproveContext) -> graph::Path {
        match self {
            Self::Rotate => path::improve::rotate,
            Self::InnerRotate => path::improve::inner_rotate,
            Self::TwoOpt => path::improve::two_opt,
            Self::ThreeOpt => path::improve::three_opt,
            Self::Swap => path::improve::swap,
            Self::SimulatedAnnealing => path::improve::simulated_annealing,
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

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub norm: Norm,
    pub invert: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    SortNumbers {
        numbers: Vec<i64>,
        algorithm: IntegerSortAlgorithm,
    },
    CreateDistPath {
        dimensions: u8,
        values: Vec<Vec<f32>>,
        #[serde(default)]
        metric: Metric,
        method: PathCreateMethod,
    },
    #[serde(rename_all = "camelCase")]
    ImproveDistPath {
        dimensions: u8,
        path: Vec<Vec<f32>>,
        #[serde(default)]
        metric: Metric,
        #[serde(default)]
        prefer_step: bool,
        method: PathImproveMethod,
    },
    CreatePath {
        matrix: Vec<Vec<f32>>,
        method: PathCreateMethod,
    },
    #[serde(rename_all = "camelCase")]
    ImprovePath {
        path: Vec<usize>,
        matrix: Vec<Vec<f32>>,
        method: PathImproveMethod,
        #[serde(default)]
        prefer_step: bool,
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
        pool: OptionsPool,
    },
    Latency,
    WordToVec {
        word: Option<String>,
        #[serde(default)]
        desc: Option<String>,
    },
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
pub enum WordToVecResult {
    Ok { vec: Vec<f32> },
    UnknownWord,
    Unsupported,
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
        numbers: Vec<i64>,
        highlight: Vec<(usize, Highlight)>,
        progress: Option<f32>,
    },
    DistPathCreation(DistPathCreation),
    DistPathImprovement(DistPathImprovement),
    PathCreation(PathCreation),
    PathImprovement(PathImprovement),
    #[serde(rename_all = "camelCase")]
    Latency {
        time_millis: u128,
    },
    WordToVec {
        word: String,
        desc: Option<String>,
        result: WordToVecResult,
    },
    RandomWord {
        word: String,
    },
}

pub trait IntoOutput {
    fn into_output(self) -> Output;
    fn relevant_information(&self) -> bool {
        true
    }
}

impl<T> From<T> for Output
where
    T: IntoOutput,
{
    fn from(value: T) -> Self {
        value.into_output()
    }
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
                        if DEBUG_WS {
                            eprintln!("RECV: {}", string);
                        }
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
    if DEBUG_WS {
        eprintln!("SEND: {}", &string);
    }
    client.send(Message::Text(string));
}

pub fn error(client: &Responder, error: Error) {
    send(client, Output::Error { error });
}
