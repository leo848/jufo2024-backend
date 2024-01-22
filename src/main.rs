#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::module_name_repetitions)]

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use action::{ActionContext, IntegerSortContext, PathCreateContext, PathImproveContext};
use dist_graph::Points;
use simple_websockets::Responder;
use typed::{Action, Output};

use crate::{
    dist_graph::Path,
    integer_sort::SortedNumbers,
    path::{creation::PathCreation, improvement::PathImprovement},
    typed::Input,
};

mod action;
mod autorestart;
mod dist_graph;
mod error;
mod graph;
mod integer_sort;
mod path;
mod typed;
mod util;

const PORT: u16 = 3141;

fn main() {
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
        Action::CreatePath {
            dimensions: dim,
            values,
            method,
            norm,
        } => {
            let method = method.implementation();
            let points = Points::try_new_raw(values, dim).expect("should send valid data");
            let ctx = PathCreateContext {
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
                dim,
                points,
                norm,
            };
            let path = method(ctx);

            typed::send(client, PathCreation::done(path));
        }
        Action::ImprovePath {
            dimensions: dim,
            path,
            method,
            norm,
        } => {
            let method = method.implementation();
            let old_path = Path::try_new_raw(path, dim).expect("should send valid data");
            let ctx = PathImproveContext {
                action: ActionContext {
                    client: client.clone(),
                    latency,
                },
                dim,
                path: old_path,
                norm,
            };
            let improved_path = method(ctx);

            typed::send(client, PathImprovement::from_path(improved_path).done());
        }
    }
}
