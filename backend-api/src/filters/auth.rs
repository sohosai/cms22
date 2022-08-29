use crate::model::User;
use firebase_admin_auth_rs::jwk_auth::JwkAuth;
use warp::Filter;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("invalid token")]
    InvalidToken,
}

impl warp::reject::Reject for Error {}

pub fn with_auth() -> warp::filters::BoxedFilter<(User,)> {
    warp::header::headers_cloned().and_then(authorize).boxed()
}

async fn authorize(
    headers: warp::http::HeaderMap<warp::http::HeaderValue>,
) -> Result<User, warp::Rejection> {
    match token_from_headers(&headers) {
        Ok(token) => {
            let jwk = JwkAuth::new("sos22-main".to_string()).await;
            let res = jwk.verify(&token);

            match res {
                Some(token_data) => Ok(User {
                    user_id: token_data.claims.sub,
                }),
                None => Err(warp::reject::custom(Error::InvalidToken)),
            }
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn token_from_headers(
    headers: &warp::http::HeaderMap<warp::http::HeaderValue>,
) -> anyhow::Result<String, Error> {
    let header = headers
        .get(warp::http::header::AUTHORIZATION)
        .ok_or(Error::NoAuthHeaderError)?;

    let value = std::str::from_utf8(header.as_bytes()).map_err(|_| Error::NoAuthHeaderError)?;

    if !value.starts_with("Bearer ") {
        return Err(Error::InvalidAuthHeaderError);
    }

    Ok(value.trim_start_matches("Bearer ").to_string())
}
