use serde::Serialize;

use crate::{dist_graph::Path, Output, typed::IntoOutput};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistPathImprovement {
    done: bool,
    better: bool,
    current_path: Path,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
}

impl DistPathImprovement {
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
}

impl IntoOutput for DistPathImprovement {
    fn into_output(self) -> Output {
        Output::DistPathImprovement(self)
    }

    fn relevant_information(&self) -> bool {
        self.better
    }
}
