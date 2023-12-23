use serde::Serialize;
use crate::{
    action::IntegerSortContext,
    typed::Highlight::{self, Compare, Correct, Swap},
    Output,
};

#[derive(Debug, Clone, Serialize)]
pub struct SortedNumbers {
    done: bool,
    numbers: Vec<u64>,
    highlight: Vec<(usize, Highlight)>,
}

impl SortedNumbers {
    pub fn new(numbers: &[u64]) -> Self {
        Self {
            numbers: numbers.to_owned(),
            done: false,
            highlight: Default::default(),
        }
    }

    pub fn done(self) -> Self {
        Self { done: true, ..self }
    }

    pub fn highlight(mut self, index: usize, highlight: Highlight) -> Self {
        self.highlight.push((index, highlight));
        self
    }

    pub fn highlights(mut self, highlights: impl IntoIterator<Item = (usize, Highlight)>) -> Self {
        for (index, highlight) in highlights {
            self = self.highlight(index, highlight);
        }
        self
    }
}

impl From<SortedNumbers> for Output {
    fn from(value: SortedNumbers) -> Self {
        Output::SortedNumbers(value)
    }
}

pub fn bubble(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlight(i, Compare)
                    .highlight(j, Compare),
            );
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                action.send(
                    SortedNumbers::new(&numbers)
                        .highlight(i, Swap)
                        .highlight(j, Swap),
                );
            }
        }

        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}

pub fn selection(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        let mut min_index = i;
        for j in i..numbers.len() {
            action
                .send(SortedNumbers::new(&numbers).highlights([(j, Compare), (min_index, Compare)]));
            if numbers[j] < numbers[min_index] {
                min_index = j;
                action.send(SortedNumbers::new(&numbers).highlight(min_index, Compare));
            }
        }
        if i != min_index {
            action.send(SortedNumbers::new(&numbers).highlights([(i, Swap), (min_index, Swap)]));
            numbers.swap(i, min_index);
            action.send(SortedNumbers::new(&numbers).highlights([(i, Swap), (min_index, Swap)]));
        } else if i != numbers.len() - 1 {
            action.send(SortedNumbers::new(&numbers).highlight(i, Compare));
        }
        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}

pub fn insertion(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        let next_to_insert = numbers[i];
        let mut index = i;
        while index > 0 && next_to_insert < numbers[index - 1] {
            action.send(
                SortedNumbers::new(&numbers).highlights([(index, Compare), (index - 1, Compare)]),
            );
            numbers.swap(index, index - 1);
            action.send(SortedNumbers::new(&numbers).highlights([(index, Swap), (index - 1, Swap)]));
            index -= 1;
        }
        if index > 0 {
            action.send(
                SortedNumbers::new(&numbers).highlights([(index, Compare), (index - 1, Compare)]),
            );
        }
        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}

pub fn merge(ctx: IntegerSortContext) -> Vec<u64> {
    todo!()
}

pub fn quick(ctx: IntegerSortContext) -> Vec<u64> {
    todo!()
}
