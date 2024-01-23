use serde::Serialize;

use crate::{graph::Path, Output};

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

    pub fn better(self, better: bool) -> Self {
        Self { better, ..self }
    }

    pub fn done(self) -> Self {
        Self {
            progress: Some(1.0),
            done: true,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn progress(self, value: f32) -> Self {
        Self {
            progress: Some(value),
            ..self
        }
    }
}

impl From<PathImprovement> for Output {
    fn from(value: PathImprovement) -> Self {
        Output::PathImprovement(value)
    }
}
