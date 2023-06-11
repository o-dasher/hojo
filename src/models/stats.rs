use bevy_reflect::Reflect;

#[derive(sqlx::FromRow, Default, Reflect)]
pub struct HojoStats {
    #[sqlx(default)]
    pub accuracy: f64,

    #[sqlx(default)]
    pub pp: f64,

    #[sqlx(default)]
    pub ranked_score: i64,

    #[sqlx(default)]
    pub total_score: i64,
}
