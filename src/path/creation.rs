use crate::Output;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathCreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    done_path: Option<Vec<Vec<f32>>>,
    current_edges: Vec<(Vec<f32>, Vec<f32>)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
}

impl PathCreation {
    pub fn from_edges(edges: Vec<(Vec<f32>, Vec<f32>)>) -> Self {
        Self {
            current_edges: edges,
            done_path: None,
            progress: None,
        }
    }

    pub fn from_path(path: Vec<Vec<f32>>) -> Self {
        Self {
            current_edges: super::path_to_edges(&path),
            done_path: None,
            progress: None,
        }
    }

    pub fn done(path: Vec<Vec<f32>>) -> Self {
        Self {
            current_edges: super::path_to_edges(&path),
            done_path: Some(path),
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

impl From<PathCreation> for Output {
    fn from(value: PathCreation) -> Self {
        Output::PathCreation(value)
    }
}
