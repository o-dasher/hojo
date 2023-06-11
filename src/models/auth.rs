use axum_login::{extractors::AuthContext, secrecy::SecretVec, AuthUser, PostgresStore};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ShionUser {
    pub id: i64,
    pub password: String,
}

impl ShionUser {
    pub fn new(id: i64, password: &str) -> Self {
        Self {
            id,
            password: password.to_string(),
        }
    }
}

impl AuthUser<i64> for ShionUser {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

pub type ShionContext = AuthContext<i64, ShionUser, PostgresStore<ShionUser>>;
