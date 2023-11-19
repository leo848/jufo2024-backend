use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Error, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Error {
    #[error("attempted to send binary data to text-only server")]
    BinaryData,
    #[error("failed to serialize: {error}")]
    Serde { original: String, error: String },
}
