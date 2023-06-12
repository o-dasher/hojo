use bevy_reflect::Reflect;
use sqlx::types::time::PrimitiveDateTime;

pub mod score;
pub mod stats;
pub mod user;

#[derive(sqlx::FromRow, Clone)]
pub struct Timestamp {
    pub value: PrimitiveDateTime,
}

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            value: PrimitiveDateTime::MIN,
        }
    }
}

#[derive(Default, sqlx::FromRow, Clone, Reflect)]
pub struct Accuracy {
    pub value: f64,
}

impl ToString for Accuracy {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Accuracy {
    pub fn as_droid(self) -> i64 {
        (self.value * 1000f64).round() as i64
    }
}
