use serde::Serialize;

use crate::{
    dist_graph::{Edge, Path},
    typed::IntoOutput,
    Output,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistPathCreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    done_path: Option<Path>,
    current_edges: Vec<Edge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
}

impl DistPathCreation {
    pub fn from_edges(edges: Vec<Edge>) -> Self {
        Self {
            current_edges: edges,
            done_path: None,
            progress: None,
        }
    }

    pub fn from_path(path: Path) -> Self {
        Self {
            current_edges: path.into_edges(),
            done_path: None,
            progress: None,
        }
    }

    pub fn done(path: Path) -> Self {
        Self {
            done_path: Some(path.clone()),
            current_edges: path.into_edges(),
            progress: Some(1.0),
        }
    }

    pub fn progress(self, value: f32) -> Self {
        Self {
            progress: Some(value),
            ..self
        }
    }
}

impl IntoOutput for DistPathCreation {
    fn into_output(self) -> Output {
        Output::DistPathCreation(self)
    }
}
