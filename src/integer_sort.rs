use serde::Serialize;

use crate::{
    typed::Highlight,
    Output,
};

mod bubble;
mod insertion;
mod merge;
mod quick;
mod selection;

pub use bubble::bubble;
pub use insertion::insertion;
pub use merge::merge;
pub use quick::quick;
pub use selection::selection;

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
