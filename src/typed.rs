use serde::{Deserialize, Serialize};
use simple_websockets::{Event, EventHub, Message, Responder};

use std::collections::HashMap;

use crate::{autorestart, error::Error, integer_sort, path};

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Algorithm {
    BubbleSort,
    SelectionSort,
    InsertionSort,
}

impl Algorithm {
    pub fn implementation(self) -> fn(&Responder, &mut [u64]) {
        match self {
            Self::BubbleSort => integer_sort::bubble,
            Self::InsertionSort => integer_sort::insertion,
            Self::SelectionSort => integer_sort::selection,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PathMethod {
    NearestNeighbor,
    BruteForce,
}

impl PathMethod {
    pub fn implementation(self) -> fn(&Responder, u8, &mut Vec<Vec<f32>>) {
        match self {
            Self::NearestNeighbor => path::create::nearest_neighbor,
            Self::BruteForce => path::create::brute_force,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    SortNumbers {
        numbers: Vec<u64>,
        algorithm: Algorithm,
    },
    CreatePath {
        dimensions: u8,
        values: Vec<Vec<f32>>,
        method: PathMethod,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    Log { message: String },
    Action { action: Action },
    Latency,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Highlight {
    Compare,
    Swap,
    Correct,
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
    #[serde(rename_all = "camelCase")]
    PathCreation {
        done: bool,
        current_path: Vec<Vec<f32>>,
    },
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

pub fn send(client: &Responder, message: Output) {
    autorestart::update();
    client.send(Message::Text(
        serde_json::to_string(&message).expect("Bug: could not serialize string"),
    ));
}

pub fn error(client: &Responder, error: Error) {
    send(client, Output::Error { error })
}
