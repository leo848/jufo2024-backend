#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::module_name_repetitions)]

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use action::{
    ActionContext, DistPathCreateContext, DistPathImproveContext, IntegerSortContext,
    PathCreateContext, PathImproveContext,
};
use graph::Graph;
use itertools::Itertools;
use path::improvement::PathImprovement;
use simple_websockets::Responder;
use typed::{Action, Output};

use crate::{
    dist_path::{creation::DistPathCreation, improvement::DistPathImprovement},
    integer_sort::SortedNumbers,
    path::creation::PathCreation,
    typed::{Input, WordToVecResult},
    word2vec::Model,
};

mod action;
#[cfg(all(unix, debug_assertions))]
mod autorestart;
#[cfg(not(all(unix, debug_assertions)))]
mod autorestart {
    pub fn update() {}
}

mod dist_graph;
mod dist_path;
mod error;
mod graph;
mod integer_sort;
mod path;
mod typed;
mod util;
mod word2vec;

const PORT: u16 = 3141;

const DEBUG_WS: bool = false;

fn main() {
    println!("Loading model. This may take some time...");
    let word_model = Model::from_file("nlp/model.bin").ok();
    println!("Model loaded successfully.");

    let event_hub = simple_websockets::launch(PORT)
        .unwrap_or_else(|_| panic!("failed to listen on port {PORT}"));
    println!("Listening on port {PORT}");

    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {
        let (client, input) = typed::poll(&event_hub, &mut clients);
        match input {
            Input::Log { message } => {
                eprintln!("Message: {message}");
            }
            Input::Action { action, latency } => {
                handle_action(action, latency, &client);
            }
            Input::Latency => {
                let time_millis = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();
                typed::send(&client, Output::Latency { time_millis });
            }
            Input::WordToVec { word, desc } => {
                if let Some(ref word_model) = word_model {
                    if let Some(word) = word {
                        word_to_vec(&word_model, word, desc, client.clone());
                    } else {
                        typed::send(
                            &client,
                            Output::RandomWord {
                                word: word_model.random_word().to_string(),
                            },
                        );
                    }
                } else {
                    typed::send(
                        &client,
                        Output::WordToVec {
                            desc,
                            word: word.unwrap_or(format!("Zufallsprinzip")),
                            result: WordToVecResult::Unsupported,
                        },
                    )
                }
            }
        }
    }
}

fn handle_action(action: Action, latency: u64, client: &Responder) {
    match action {
        Action::SortNumbers { numbers, algorithm } => {
            let method = algorithm.implementation();
            let ctx = IntegerSortContext {
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
                numbers,
            };
            let numbers = method(ctx);

            typed::send(client, SortedNumbers::new(&numbers).done());
        }
        Action::CreateDistPath {
            dimensions: dim,
            values,
            method,
            metric,
        } => {
            let method = method.dist_implementation();
            let points = values
                .into_iter()
                .map(|v| dist_graph::Point::new(v))
                .collect_vec();
            let ctx = DistPathCreateContext {
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
                dim,
                points: points.clone(),
                graph: Graph::from_points(points, metric),
                metric,
            };
            let path = method(ctx);

            typed::send(client, DistPathCreation::done(path));
        }
        Action::ImproveDistPath {
            dimensions: dim,
            path,
            prefer_step,
            method,
            metric,
        } => {
            let method = method.dist_implementation();
            let old_path =
                dist_graph::Path::try_new_raw(path, dim).expect("should send valid data");
            let ctx = DistPathImproveContext {
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
                dim,
                prefer_step,
                path: old_path.clone(),
                graph: Graph::from_points(old_path.into_inner(), metric),
                metric,
            };
            let improved_path = method(ctx.clone());

            ctx.action
                .send(DistPathImprovement::from_path(improved_path.clone()).progress(1.0));
            ctx.action
                .send(DistPathImprovement::from_path(improved_path).done());
        }
        Action::CreatePath { matrix, method } => {
            let method = method.implementation();
            let input_graph = Graph::from_values(matrix).expect("Invalid matrix");
            let ctx = PathCreateContext {
                graph: input_graph,
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
            };
            let path = method(ctx);
            typed::send(client, PathCreation::done(path));
        }
        Action::ImprovePath {
            path,
            matrix,
            method,
            prefer_step,
        } => {
            let method = method.implementation();
            let input_graph = Graph::from_values(matrix).expect("invalid matrix");
            let old_path = graph::Path::new(path);
            let ctx = PathImproveContext {
                graph: input_graph,
                path: old_path,
                prefer_step,
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
            };
            let improved_path = method(ctx.clone());

            ctx.action
                .send(PathImprovement::from_path(improved_path.clone()).progress(1.0));
            ctx.action
                .send(PathImprovement::from_path(improved_path).done());
        }
    }
}

fn word_to_vec(word_model: &Model, word: String, desc: Option<String>, client: Responder) {
    let vec_for = word_model.vec_for(&word);
    let result = match vec_for {
        Ok(vec) => WordToVecResult::Ok {
            vec: vec.into_inner(),
        },
        Err(error) => match error {
            word2vec::Error::NotInVocabulary(_) => WordToVecResult::UnknownWord,
            word2vec::Error::Word2Vec(_) | word2vec::Error::Io(_) => panic!("{}", error),
        },
    };

    typed::send(&client, Output::WordToVec { word, result, desc })
}
