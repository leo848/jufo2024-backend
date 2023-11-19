use simple_websockets::Responder;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::{collections::HashMap, time::UNIX_EPOCH};
use typed::{Action, Output};

use crate::typed::Input;

mod autorestart;
mod error;
mod integer_sort;
mod path;
mod typed;

const PORT: u16 = 3141;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms))
}

fn main() {
    let event_hub =
        simple_websockets::launch(PORT).expect(&format!("failed to listen on port {PORT}"));
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
                typed::send(&client, Output::Latency { time_millis })
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
                &client,
                Output::SortedNumbers {
                    done: true,
                    numbers,
                    highlight: vec![],
                },
            );
        }
        Action::CreatePath {
            dimensions: dim,
            mut values,
            method,
        } => {
            let method = method.implementation();
            method(client, dim, &mut *values);
        }
    }
}
