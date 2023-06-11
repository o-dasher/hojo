use anyhow::Error;
use axum::{Extension, Form};
use bevy_reflect::GetField;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

use crate::{
    models::{
        auth::{ShionContext, ShionUser},
        user::MeiUser,
    },
    requests::{
        payload::{Password, Username},
        response::{Rena, RenaResponse, ToRenaInner},
    },
    utils::config::Config,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn generic_error(err: impl Into<Error>) -> Rena<'static> {
    Rena::no(err, vec!["Failed to submit score"])
}

pub async fn login_route(
    mut auth: ShionContext,
    Extension(config): Extension<Config>,
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<LoginPayload>,
) -> RenaResponse {
    let username = Username::new(payload.username)
        .unwrap_rena(())?
        .into_inner();

    let password = Password::new(payload.password)
        .unwrap_rena(())?
        .into_inner();

    let metric_key = config.global_metric.to_string();

    let login_user: MeiUser = sqlx::query_as(
        "
        SELECT
        u.id, u.name, u.password, RANK() OVER (ORDER BY $1 DESC) as rank,
        s.play_count, s.pp, s.ranked_score, s.total_score, s.accuracy
        FROM osu_user u
        JOIN osu_stats s ON u.id = s.user_id
        WHERE u.name = $2
        ",
    )
    .bind(&metric_key)
    .bind(username)
    .fetch_one(&pool)
    .await
    .map_err(|e| Rena::no(e, vec!["User not found"]))?;

    bcrypt::verify(password, &login_user.password)
        .map_err(|e| Rena::no(e, vec!["Password does not match"]))?;

    auth.login(&ShionUser::new(login_user.id, &login_user.password))
        .await
        .map_err(|e| Rena::no(anyhow::anyhow!("{}", e), vec!["Could not login"]))?;

    info!("{}", metric_key);

    let metric: &i64 = login_user.stats
        .get_field(&metric_key)
        .ok_or(Rena::unchecked_fail(vec!["Failed to get user metric"]))?;

    Rena::ok(vec![
        &login_user.id.to_string(),
        Default::default(),
        &login_user.rank.to_string(),
        &metric.to_string(),
        &login_user.stats.accuracy.to_string(),
        &login_user.name,
        &String::default(),
    ])
    .into()
}
