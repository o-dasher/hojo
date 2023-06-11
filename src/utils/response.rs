use anyhow::Error;
use axum::response::IntoResponse;
use strum::Display;

#[derive(Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
pub enum RenaType {
    Sucess,
    Failed,
}

pub type RenaArgs<'a> = Vec<&'a str>;

pub struct Rena<'a>(RenaType, RenaArgs<'a>);

impl<'a> Rena<'a> {
    pub fn ok(args: RenaArgs<'a>) -> Self {
        Self(RenaType::Sucess, args)
    }

    pub fn no(err: impl Into<Error>, args: RenaArgs<'a>) -> Self {
        let err: Error = err.into();

        tracing::info!("{}", &err);

        Self::unchecked_fail(args)
    }

    pub fn unchecked_fail(args: RenaArgs<'a>) -> Self {
        Self(RenaType::Failed, args)
    }
}

impl ToString for Rena<'_> {
    fn to_string(&self) -> String {
        format!("{}\n{}", self.0, self.1.join(" "))
    }
}

impl From<Rena<'_>> for String {
    fn from(val: Rena<'_>) -> Self {
        val.to_string()
    }
}

impl IntoResponse for Rena<'_> {
    fn into_response(self) -> axum::response::Response {
        self.to_string().into_response()
    }
}

pub type RenaResult = Result<String, String>;

pub trait ToRena<'a, B> {
    fn to_rena(self, args: B) -> Rena<'a>;
}

impl From<Rena<'_>> for RenaResult {
    fn from(val: Rena<'_>) -> Self {
        match val.0 {
            RenaType::Sucess => Ok(val.to_string()),
            RenaType::Failed => Err(val.to_string()),
        }
    }
}
