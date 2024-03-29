use serde::Serialize;

use crate::{graph::Path, Output, typed::IntoOutput};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathImprovement {
    done: bool,
    better: bool,
    current_path: Path,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
}

impl PathImprovement {
    pub fn from_path(path: Path) -> Self {
        Self {
            current_path: path,
            better: true,
            done: false,
            progress: None,
        }
    }

    pub fn done(self) -> Self {
        Self {
            progress: Some(1.0),
            done: true,
            ..self
        }
    }

    pub fn progress(self, value: f32) -> Self {
        Self {
            progress: Some(value),
            ..self
        }
    }

    pub fn not_better(self) -> Self {
        Self {
            better: false,
            ..self
        }
    }
}

impl IntoOutput for PathImprovement {
    fn into_output(self) -> Output {
        Output::PathImprovement(self)
    }

    fn relevant_information(&self) -> bool {
        self.better
    }
}
