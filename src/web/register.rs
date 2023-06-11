use anyhow::Error;
use axum::{Extension, Form};
use bcrypt::DEFAULT_COST;
use serde::Deserialize;
use sqlx::PgPool;

use crate::utils::{
    payload::{Email, Password, Username},
    response::{Rena, ToRenaInner},
};

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    password: String,
    email: String,
}

fn generic_fail(err: impl Into<Error>) -> Rena<'static> {
    Rena::no(err, vec!["Failed to register user"])
}

pub async fn register_route(
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<RegisterPayload>,
) -> Result<String, String> {
    let email = Email::new(payload.email).into_inner();

    let username = Username::new(payload.username)
        .unwrap_rena(())?
        .into_inner();

    let password = Password::new(payload.password)
        .unwrap_rena(())?
        .into_inner();

    let existing_user = sqlx::query!(
        "SELECT name, email FROM osu_user WHERE name=$1 or email=$2",
        username,
        email
    )
    .fetch_one(&pool)
    .await;

    if let Ok(existing) = existing_user {
        let response = {
            if existing.name == username {
                "This username is already taken"
            } else {
                "This email is already in use"
            }
        };

        return Rena::unchecked_fail(vec![response]).into();
    }

    let hashed_password = bcrypt::hash(&password, DEFAULT_COST).map_err(generic_fail)?;

    sqlx::query!(
        "
        WITH created_user AS (
            INSERT INTO osu_user (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING id
        )
        INSERT INTO osu_stats (user_id) VALUES (
            (SELECT created_user.id FROM created_user)
        );
        ",
        username,
        email,
        hashed_password
    )
    .execute(&pool)
    .await
    .map_err(generic_fail)?;

    Rena::ok(vec!["Account created"]).into()
}
