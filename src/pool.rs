use crate::path::create::ilp::MilpSolver;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionsPool {
    #[serde(default)]
    pub iteration_count: Option<usize>,
    #[serde(default)]
    pub initial_temperature: Option<f64>,
    #[serde(default)]
    pub milp_solver: Option<MilpSolver>,
    #[serde(default)]
    pub ilp_max_duration: Option<u64>,
    #[serde(default)]
    pub ilp_start: Option<usize>,
    #[serde(default)]
    pub ilp_end: Option<usize>,
}
