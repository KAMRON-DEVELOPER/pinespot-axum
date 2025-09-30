use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;

use crate::utilities::errors::AppError;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleOAuthUserSub(pub String);

impl<S> FromRequestParts<S> for GoogleOAuthUserSub
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        if let Some(cookie) = jar.get("google_oauth_user_sub") {
            let google_oauth_user_sub = cookie.value();
            return Ok(Self(google_oauth_user_sub.to_owned()));
        }

        Err(AppError::MissingGoogleOAuthSubError)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionalGoogleOAuthUserSub(pub Option<String>);

impl<S> FromRequestParts<S> for OptionalGoogleOAuthUserSub
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        if let Some(cookie) = jar.get("google_oauth_user_sub") {
            let google_oauth_user_sub = cookie.value();
            return Ok(Self(Some(google_oauth_user_sub.to_owned())));
        }

        Ok(Self(None))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubOAuthUserId(pub i64);

impl<S> FromRequestParts<S> for GithubOAuthUserId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        if let Some(cookie) = jar.get("github_oauth_user_id") {
            let github_oauth_user_id = cookie.value().parse::<i64>().map_err(|_| {
                AppError::ValidationError("Github oauth user id is not integer".to_string())
            })?;
            return Ok(Self(github_oauth_user_id));
        }

        Err(AppError::MissingGithubOAuthIdError)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionalGithubOAuthUserId(pub Option<i64>);

impl<S> FromRequestParts<S> for OptionalGithubOAuthUserId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        if let Some(cookie) = jar.get("github_oauth_user_id") {
            let github_oauth_user_id = cookie.value().parse::<i64>().map_err(|_| {
                AppError::ValidationError("Github oauth user id is not integer".to_string())
            })?;
            return Ok(Self(Some(github_oauth_user_id)));
        }

        Ok(Self(None))
    }
}
