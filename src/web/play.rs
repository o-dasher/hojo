use axum::{Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::user::HoUser,
    requests::response::{Rena, RenaResponse},
};

#[derive(Deserialize)]
pub struct PlayPayload {
    hash: String,
}

pub async fn play_route(
    Extension(user): Extension<HoUser>,
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<PlayPayload>,
) -> RenaResponse {
    sqlx::query!(
        "
        UPDATE users 
        SET playing = $1
        WHERE id = $2
        ",
        payload.hash,
        user.id
    )
    .execute(&pool)
    .await
    .map_err(|e| Rena::no(e, vec![]).into())
    .map(|_r| Rena::ok(vec![&(true as i32).to_string(), &user.id.to_string()]).into())
}

