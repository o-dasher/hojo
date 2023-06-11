use serde::Deserialize;
use strum::{Display, IntoStaticStr};

#[derive(Default, Deserialize, Clone, Display, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum GlobalMetric {
    Pp,
    #[default]
    TotalScore,
    RankedScore,
}

#[derive(Default, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ScoreMetric {
    #[default]
    Pp,
    Score,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "Default::default")]
    pub is_production: bool,

    #[serde(default)]
    pub score_metric: ScoreMetric,

    #[serde(default)]
    pub global_metric: GlobalMetric,
}
