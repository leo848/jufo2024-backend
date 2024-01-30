use derive_more::Constructor;
use serde::Serialize;

#[derive(Debug, Constructor, Copy, Clone, Serialize)]
pub struct Edge(pub usize, pub usize);
