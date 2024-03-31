use core::ops::Range;
use std::collections::HashMap;

use serde::Serialize;

use crate::{
    typed::{Highlight, Highlight::Consider, IntoOutput},
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
    #[serde(default)]
    progress: Option<f32>,
    numbers: Vec<i64>,
    highlight: HashMap<usize, Highlight>,
}

impl SortedNumbers {
    pub fn new(numbers: &[i64]) -> Self {
        Self {
            numbers: numbers.to_owned(),
            progress: None,
            done: false,
            highlight: Default::default(),
        }
    }

    pub fn progress(self, progress: f32) -> Self {
        Self {
            progress: Some(progress),
            ..self
        }
    }

    pub fn done(self) -> Self {
        Self {
            done: true,
            progress: Some(1.0),
            ..self
        }
    }

    pub fn highlight(mut self, index: usize, highlight: Highlight) -> Self {
        self.highlight.insert(index, highlight);
        self
    }

    pub fn highlights(mut self, highlights: impl IntoIterator<Item = (usize, Highlight)>) -> Self {
        for (index, highlight) in highlights {
            self = self.highlight(index, highlight);
        }
        self
    }

    pub fn consider(self, range: &Range<usize>) -> Self {
        self.highlights(range.clone().map(|i| (i, Consider)))
    }
}

impl IntoOutput for SortedNumbers {
    fn into_output(self) -> Output {
        let Self {
            done,
            numbers,
            highlight,
            progress,
        } = self;
        Output::SortedNumbers {
            done,
            numbers,
            highlight: highlight.into_iter().collect(),
            progress,
        }
    }
}
