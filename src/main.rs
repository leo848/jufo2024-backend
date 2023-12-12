#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::module_name_repetitions)]

use std::{
    collections::HashMap,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use graph::Points;
use simple_websockets::Responder;
use typed::{Action, Output};

use crate::{
    graph::Path,
    path::{creation::PathCreation, improvement::PathImprovement},
    typed::Input,
};

mod autorestart;
mod error;
mod graph;
mod integer_sort;
mod path;
mod typed;
mod util;

const PORT: u16 = 3141;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

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
            Input::Action { action } => {
                handle_action(action, &client);
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

fn handle_action(action: Action, client: &Responder) {
    match action {
        Action::SortNumbers {
            mut numbers,
            algorithm,
        } => {
            let method = algorithm.implementation();
            method(client, &mut numbers);

            typed::send(
                client,
                Output::SortedNumbers {
                    done: true,
                    numbers,
                    highlight: vec![],
                },
            );
        }
        Action::CreatePath {
            dimensions: dim,
            values,
            method,
        } => {
            let method = method.implementation();
            let points = Points::try_new_raw(values, dim).expect("should send valid data");
            let path = method(client, dim, points);

            typed::send(client, PathCreation::done(path));
        }
        Action::ImprovePath {
            dimensions: dim,
            path,
            method,
        } => {
            let method = method.implementation();
            let old_path = Path::try_new_raw(path, dim).expect("should send valid data");
            let improved_path = method(client, dim, old_path.clone());

            typed::send(client, PathImprovement::from_path(improved_path).done());
        }
    }
}
