use axum_login::{extractors::AuthContext, secrecy::SecretVec, AuthUser, PostgresStore};
use bevy_reflect::Reflect;

use super::{stats::MeguriStats, Timestamp};

// Meguri represents the full part

#[derive(Default, sqlx::FromRow)]
pub struct MeguriUser {
    #[sqlx(flatten, default)]
    pub mei: MeiUser,
    #[sqlx(flatten, default)]
    pub gou: GouUser,
}

// -- Mei represents the part that can be reflected and has relationship

#[derive(Default, Reflect, sqlx::FromRow)]
pub struct MeiUser {
    #[sqlx(flatten, default)]
    pub ho: HoUser,
    #[sqlx(flatten, default)]
    pub stats: MeguriStats,
}

// -- Ho represents the part that is core of the table

#[derive(Default, Reflect, sqlx::FromRow, Clone)]
pub struct HoUser {
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
}

impl AuthUser<i64> for HoUser {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

pub type ShionContext = AuthContext<i64, HoUser, PostgresStore<HoUser>>;

// -- Gou represents the part that can't be reflected

#[derive(Default, sqlx::FromRow)]
pub struct GouUser {
    #[sqlx(flatten, default)]
    pub created_at: Timestamp,
    #[sqlx(flatten, default)]
    pub last_seen: Timestamp,
}
