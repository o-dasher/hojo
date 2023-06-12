use anyhow::Error;
use axum::{Extension, Form};
use bevy_reflect::GetField;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

use crate::{
    models::user::{MeiUser, ShionContext, HoUser},
    requests::{
        payload::{Password, Username},
        response::{Rena, RenaResponse},
    },
    utils::config::Config,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn generic_error(err: impl Into<Error>) -> Rena<'static> {
    Rena::no(err, vec!["Failed to login"])
}

pub fn invalid_user(err: impl Into<Error>) -> Rena<'static> {
    Rena::no(err, vec!["User not found"])
}

pub fn invalid_password(err: impl Into<Error>) -> Rena<'static> {
    Rena::no(err, vec!["Password does not match"])
}

pub async fn login_route(
    mut auth: ShionContext,
    Extension(config): Extension<Config>,
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<LoginPayload>,
) -> RenaResponse {
    // Same as below
    let username = Username::new(payload.username)
        .map_err(invalid_user)?
        .into_inner();

    // We must not give out the reason why the password wasn't accepted.
    // so we map it to a generic "Password does not match" error
    let password = Password::new(payload.password)
        .map_err(invalid_password)?
        .into_inner();

    let metric_key = config.global_metric.to_string();

    let login_user: MeiUser = sqlx::query_as(
        "
        SELECT
        u.id, u.name, u.password, RANK() OVER (ORDER BY $1 DESC) as rank,
        s.play_count, s.pp, s.ranked_score, s.total_score, s.accuracy
        FROM users u
        JOIN osu_stats s ON u.id = s.user_id
        WHERE u.name = $2
        ",
    )
    .bind(&metric_key)
    .bind(username)
    .fetch_one(&pool)
    .await
    .map_err(invalid_user)?;

    bcrypt::verify(password, &login_user.ho.password).map_err(invalid_password)?;

    auth.login(&login_user.ho)
        .await
        .map_err(|e| Rena::no(anyhow::anyhow!("{}", e), vec!["Could not login"]))?;

    info!("{}", metric_key);

    let metric: &i64 = login_user
        .stats
        .get_field(&metric_key)
        .ok_or(Rena::unchecked_fail(vec!["Failed to get user metric"]))?;

    Rena::ok(vec![
        &login_user.ho.id.to_string(),
        Default::default(),
        &login_user.ho.rank.to_string(),
        &metric.to_string(),
        &login_user.stats.accuracy.as_droid().to_string(),
        &login_user.ho.name,
        &String::default(),
    ])
    .into()
}
