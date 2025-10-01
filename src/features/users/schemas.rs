use crate::features::users::models::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RedirectResponse {
    pub redirect_to: String,
}

#[derive(Deserialize, Debug)]
pub struct OAuthCallback {
    pub(crate) code: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhoneResponse {
    pub phone_numbers: Option<Vec<PhoneNumber>>,
}

#[derive(Deserialize, Debug)]
pub struct PhoneNumber {
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct AuthResponse {
    pub user: User,
    pub tokens: Tokens,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Deserialize, Validate, Debug)]
pub struct ContinueWithEmailSchema {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 32,
        message = "Password should be long beetween 8 and 32"
    ))]
    pub password: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct GoogleOAuthUser {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct GithubOAuthUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct OAuthUserSchema {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub picture: Option<String>,
}
