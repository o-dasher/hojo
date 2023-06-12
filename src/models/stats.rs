use bevy_reflect::Reflect;

use super::Accuracy;

#[derive(sqlx::FromRow, Default, Reflect)]
pub struct MeguriStats {
    #[sqlx(flatten, default)]
    pub accuracy: Accuracy,

    #[sqlx(default)]
    pub pp: f64,

    #[sqlx(default)]
    pub ranked_score: i64,

    #[sqlx(default)]
    pub total_score: i64,
}
