use bevy_reflect::Reflect;

use super::{stats::HojoStats, Timestamp};

#[derive(Default, sqlx::FromRow)]
pub struct FullUser(MeiUser, GouUser);

#[derive(Default, Reflect, sqlx::FromRow)]
pub struct MeiUser {
    #[sqlx(default)]
    pub id: i64,
    #[sqlx(default)]
    pub name: String,
    #[sqlx(default)]
    pub email: String,
    #[sqlx(default)]
    pub password: String,
    #[sqlx(default)]
    pub rank: i64,
    #[sqlx(default)]
    pub playing: Option<String>,
    #[sqlx(default)]
    pub play_count: i64,

    #[sqlx(flatten, default)]
    pub stats: HojoStats,
}

// -- Gou is the name for the part of the struct that can't implement reflection

#[derive(Default, sqlx::FromRow)]
pub struct GouUser {
    #[sqlx(flatten, default)]
    pub created_at: Timestamp,
    #[sqlx(flatten, default)]
    pub last_seen: Timestamp,
}
