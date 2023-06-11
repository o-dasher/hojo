use sqlx::types::time::PrimitiveDateTime;

pub mod auth;
pub mod score;
pub mod user;
pub mod stats;

#[derive(sqlx::FromRow, Clone)]
pub struct Timestamp(PrimitiveDateTime); 

impl Default for Timestamp {
    fn default() -> Self {
        Self(PrimitiveDateTime::MIN)
    }
}

