use derivative::Derivative;
use rosu_v2::prelude::{GameMode, ScoreStatistics};

use super::Timestamp;

#[derive(Clone, Derivative, Default, sqlx::FromRow)]
pub struct HojoScore {
    pub id: i64,
    pub beatmap_id: i64,
    pub user_id: i64,
    pub pp: f64,
    pub score: i64,
    pub combo: i64,
    pub perfect: i64,
    pub good: i64,
    pub bad: i64,
    pub miss: i64,
    pub geki: i64,
    pub katu: i64,
    #[sqlx(flatten)]
    pub date: Timestamp,
}

impl From<HojoScore> for ScoreStatistics {
    fn from(val: HojoScore) -> Self {
        Self {
            count_geki: val.geki as u32,
            count_300: val.perfect as u32,
            count_katu: val.katu as u32,
            count_100: val.good as u32,
            count_50: val.bad as u32,
            count_miss: val.miss as u32,
        }
    }
}

pub trait QuickAccuracy {
    fn quick_acc(self) -> f32;
}

impl QuickAccuracy for ScoreStatistics {
    fn quick_acc(self) -> f32 {
        self.accuracy(GameMode::Osu)
    }
}
