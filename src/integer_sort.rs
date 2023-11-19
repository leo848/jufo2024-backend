use crate::typed::{
    self,
    Highlight::{self, Compare, Correct, Swap},
};
use crate::Output;
use simple_websockets::Responder;
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms))
}

fn send(
    client: &Responder,
    numbers: &[u64],
    highlight: impl IntoIterator<Item = (usize, Highlight)>,
) {
    typed::send(
        &client,
        Output::SortedNumbers {
            done: false,
            numbers: numbers.into(),
            highlight: highlight.into_iter().collect(),
        },
    )
}

pub fn bubble(client: &Responder, numbers: &mut [u64]) {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            send(&client, numbers, vec![(i, Compare), (j, Compare)]);
            sleep_ms(500);
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                send(&client, numbers, vec![(i, Swap), (j, Swap)]);
                sleep_ms(500);
            }
        }

        send(&client, numbers, (0..=i).map(|i| (i, Correct)));
        sleep_ms(750);
    }
}

pub fn selection(client: &Responder, numbers: &mut [u64]) {
    for i in 0..numbers.len() {
        let mut min_index = i;
        for j in i..numbers.len() {
            send(&client, numbers, vec![(j, Compare), (min_index, Compare)]);
            sleep_ms(500);
            if numbers[j] < numbers[min_index] {
                min_index = j;
                send(&client, numbers, vec![(min_index, Compare)]);
                sleep_ms(500);
            }
        }
        if i != min_index {
            send(&client, numbers, vec![(i, Swap), (min_index, Swap)]);
            sleep_ms(500);
            numbers.swap(i, min_index);
            send(&client, numbers, vec![(i, Swap), (min_index, Swap)]);
            sleep_ms(500);
        } else if i != numbers.len()-1 {
            send(&client, numbers, vec![(i, Compare)]);
            sleep_ms(500);
        }
        send(
            &client,
            numbers,
            (0..=i).map(|i|(i, Correct))
        );
        sleep_ms(500);
    }
}

pub fn insertion(client: &Responder, numbers: &mut [u64]) {
    for i in 0..numbers.len() {
        let next_to_insert = numbers[i];
        let mut index = i;
        while index > 0 && next_to_insert < numbers[index - 1] {
            send(
                &client,
                numbers,
                vec![(index, Compare), (index - 1, Compare)],
            );
            sleep_ms(500);
            numbers.swap(index, index - 1);
            send(&client, numbers, vec![(index, Swap), (index - 1, Swap)]);
            sleep_ms(500);
            index -= 1;
        }
        if index > 0 {
            send(
                &client,
                numbers,
                vec![(index, Compare), (index - 1, Compare)],
            );
            sleep_ms(500);
        }
        send(&client, numbers, (0..=i).map(|i| (i, Correct)));
        sleep_ms(750);
    }
}
