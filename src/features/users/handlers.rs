use crate::{
    features::users::schemas::{AuthResponse, CompleteProfileSchema, Tokens},
    services::google_oauth::GoogleOAuthClient,
    utilities::{
        config::Config, cookie::{GoogleOAuthUserSub,   OptionalGoogleOAuthUserSub}, errors::AppError, jwt::{Claims, TokenType, create_token, verify_token}
    },
};
use bcrypt::{DEFAULT_COST, hash}; 
use std::net::SocketAddr;

use object_store::path::Path as ObjectStorePath;

use cookie::{
    SameSite,
    time::{Duration as CookieDuration },
};

use axum::{
    Json,
    extract::{ConnectInfo, Multipart, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::{
    TypedHeader,
    extract::{PrivateCookieJar, cookie::Cookie},
    headers::{Authorization, UserAgent, authorization::Bearer},
};
use chrono::{ Utc};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};

use object_store::{ObjectStore, gcp::GoogleCloudStorage};
use reqwest::Client;
use tracing::debug;
use uuid::Uuid;

use crate::{
    features::users::{
        models::{GoogleOAuthUser, User, UserRole, UserStatus},
        schemas::{LoginSchema, OAuthCallback, PhoneResponse},
    },
    services::database::Database,
};

pub async fn google_oauth_handler(
    jar: PrivateCookieJar,
    State(config): State<Config>,
    OptionalGoogleOAuthUserSub(optional_google_user_sub): OptionalGoogleOAuthUserSub,
    State(oauth_client): State<GoogleOAuthClient>,
) -> Result<(PrivateCookieJar, Redirect), AppError> {
    if optional_google_user_sub.is_some() {
        return Ok((jar, Redirect::to("http://localhost:5173/complete-profile")));
    }

    // No cookie, start OAuth flow
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/user.phonenumbers.read".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let pkce_verifier_cookie =
        Cookie::build(("pkce_verifier", pkce_code_verifier.secret().to_string()))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::days(365))
        .secure(config.cookie_secure.unwrap_or(true));
    let jar = jar.add(pkce_verifier_cookie);

    Ok((jar, Redirect::to(auth_url.as_ref())))
}

pub async fn google_oauth_callback_handler(
    jar: PrivateCookieJar,
    State(http_client): State<Client>,
    State(database): State<Database>,
    State(config): State<Config>,
    Query(query): Query<OAuthCallback>,
    State(oauth_client): State<GoogleOAuthClient>,
) -> Result<(PrivateCookieJar, Redirect), AppError> {
    let pkce_verifier = jar
        .get("pkce_verifier")
        .map(|cookie| PkceCodeVerifier::new(cookie.value().to_string()))
        .ok_or(AppError::MissingPkceCodeVerifierError)?;
 
    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    let access_token = token_response.access_token().secret();

    let get_google_oauth_user_response = http_client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(access_token.clone())
        .send()
        .await?;
    debug!("get_google_oauth_user_response: {:#?}", get_google_oauth_user_response);
 
    let google_oauth_user = get_google_oauth_user_response.json::<GoogleOAuthUser>().await?;
    debug!("google_oauth_user: {:#?}", google_oauth_user);

    let get_phone_number_response = http_client
        .get("https://people.googleapis.com/v1/people/me?personFields=phoneNumbers")
        .bearer_auth(access_token.clone())
        .send()
        .await?;
    let phone_number = get_phone_number_response.json::<PhoneResponse>().await?;
    debug!("phone number: {:?}", phone_number);

    let phone_number = phone_number
        .phone_numbers
        .as_ref()
        .and_then(|v| v.first())
        .map(|p| &p.value);
  
    let google_oauth_user_sub = sqlx::query_scalar!(
        r#"
            INSERT INTO google_oauth_users (sub , email,email_verified, family_name, given_name, name, picture, phone_number)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING sub
        "#,
        google_oauth_user.sub, 
        google_oauth_user.email,
        google_oauth_user.email_verified,
        google_oauth_user.family_name,
        google_oauth_user.given_name,
        google_oauth_user.name,
        google_oauth_user.picture,
        phone_number
    )
    .fetch_one(&database.pool)
    .await?;
  

    let google_oauth_user_sub_cookie = Cookie::build(("google_oauth_user_sub", google_oauth_user_sub))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(CookieDuration::days(365))
    .secure(config.cookie_secure.unwrap_or(true));
    let jar = jar.add(google_oauth_user_sub_cookie);


    Ok((jar, Redirect::to("http://localhost:5173/complete-profile")))
}

pub async fn get_google_oauth_user_handler(
    GoogleOAuthUserSub(google_oauth_user_sub): GoogleOAuthUserSub,
    State(database): State<Database>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_user = sqlx::query_as!(
        GoogleOAuthUser,
        r#"
            SELECT * FROM google_oauth_users WHERE sub = $1
        "#,
        google_oauth_user_sub
    )
    .fetch_optional(&database.pool)
    .await?
    .ok_or(AppError::GoogleOAuthUserNotFoundError)?;

    Ok(Json(oauth_user))
}

pub async fn complete_profile_handler(
    jar: PrivateCookieJar,
    GoogleOAuthUserSub(google_oauth_user_sub): GoogleOAuthUserSub,
    State(gcs): State<GoogleCloudStorage>,
    State(database): State<Database>,
    State(config): State<Config>,
    mut multipart: Multipart,
    // State(s3): State<AmazonS3>,
    // TypedHeader(_user_agent): TypedHeader<UserAgent>,
    // ConnectInfo(_addr): ConnectInfo<SocketAddr>,
) -> Result<(PrivateCookieJar, impl IntoResponse), AppError> { 

    let _google_oauth_user = sqlx::query_as!(
        GoogleOAuthUser,
        r#"
            SELECT * FROM google_oauth_users WHERE sub = $1
        "#,
        google_oauth_user_sub
    )
    .fetch_optional(&database.pool)
    .await?
    .ok_or(AppError::GoogleOAuthUserNotFoundError)?;

   

    let mut complete_profile_schema = CompleteProfileSchema {
        given_name: None,
        family_name: None,
        email: None,
        password: None,
        phone_number: None,
        picture: None,
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "given_name" => {
                complete_profile_schema.given_name = Some(field.text().await.unwrap());
            }
            "family_name" => {
                complete_profile_schema.family_name = Some(field.text().await.unwrap());
            }
            "email" => {
                complete_profile_schema.email = Some(field.text().await.unwrap());
            }
            "password" => {
                complete_profile_schema.password = Some(field.text().await.unwrap());
            }
            "phone_number" => {
                complete_profile_schema.phone_number = Some(field.text().await.unwrap());
            }
            "picture" => {
                let data = field.bytes().await.unwrap();
                let pic_id = Uuid::new_v4();
                let ext = infer::get(&data)
                    .ok_or_else(|| {
                        AppError::InvalidImageFormatError("Invalid image format".to_string())
                    })?
                    .extension();
                let location =
                    ObjectStorePath::from(format!("{}/{}.{}", google_oauth_user_sub, pic_id, ext));
                gcs.put(&location, data.into()).await?;
                complete_profile_schema.picture = Some(location);
            }
            _ => {}
        }
    }

    debug!("complete_profile_schema: {:#?}", complete_profile_schema);

    let picture = complete_profile_schema.picture.map(|p| p.to_string());
    let hash_password = hash(complete_profile_schema.password.unwrap(), DEFAULT_COST)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (first_name, last_name, email, phone_number, password, picture)
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING
            id,
            first_name,
            last_name,
            email,
            phone_number,
            password,
            picture,
            role AS "role: UserRole",
            status AS "status: UserStatus",
            created_at,
            updated_at
        "#,
        complete_profile_schema.given_name.unwrap(),
        complete_profile_schema.family_name.unwrap(),
        complete_profile_schema.email.unwrap(),
        complete_profile_schema.phone_number,
        hash_password,
        picture
    )
    .fetch_one(&database.pool)
    .await?;

    let new_access = create_token(&config, user.id, false)?;
    let new_refresh = create_token(&config, user.id, true)?;

    let max_age_days = config.refresh_token_expire_in_days.unwrap_or(30);
    let refresh_cookie = Cookie::build(("refresh_token", new_refresh.clone()))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::days(max_age_days))
        .secure(config.cookie_secure.unwrap_or(true));

    let jar = jar.add(refresh_cookie);

    let tokens = Tokens {
        access_token: new_access,
        refresh_token: Some(new_refresh),
    };
    let response = Json(AuthResponse { user, tokens });
    Ok((jar, response))
}

pub async fn signup_handler(
    jar: PrivateCookieJar,
    State(database): State<Database>,
    State(config): State<Config>,
    TypedHeader(_user_agent): TypedHeader<UserAgent>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Json(login_schema): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    debug!("login_schema is {:#?}", login_schema);

    let user = login_schema.verify(&database).await?.ok_or_else(|| {
        AppError::NotFoundError("User not found with this username and password".to_string())
    })?;

    let new_access = create_token(&config, user.id, false)?;
    let new_refresh = create_token(&config, user.id, true)?;

    let max_age_days = config.refresh_token_expire_in_days.unwrap_or(30);
    let refresh_cookie = Cookie::build(("refresh_token", new_refresh.clone()))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(CookieDuration::days(max_age_days))
    .secure(config.cookie_secure.unwrap_or(true));

    let jar = jar.add(refresh_cookie);

    let tokens = Tokens {
        access_token: new_access,
        refresh_token: Some(new_refresh),
    };
    let response = Json(AuthResponse { user, tokens });
    Ok((jar, response))
}

pub async fn verification_handler(
    jar: PrivateCookieJar,
    State(database): State<Database>,
    State(config): State<Config>,
    TypedHeader(_user_agent): TypedHeader<UserAgent>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Json(login_schema): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    debug!("login_schema is {:#?}", login_schema);

    let user = login_schema.verify(&database).await?.ok_or_else(|| {
        AppError::NotFoundError("User not found with this username and password".to_string())
    })?;

    let new_access = create_token(&config, user.id, false)?;
    let new_refresh = create_token(&config, user.id, true)?;

    let max_age_days = config.refresh_token_expire_in_days.unwrap_or(30);
    let refresh_cookie = Cookie::build(("refresh_token", new_refresh.clone()))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::days(max_age_days))
        .secure(config.cookie_secure.unwrap_or(true));

    let jar = jar.add(refresh_cookie);

    let tokens = Tokens {
        access_token: new_access,
        refresh_token: Some(new_refresh),
    };
    let response = Json(AuthResponse { user, tokens });
    Ok((jar, response))
}

pub async fn signin_handler(
    jar: PrivateCookieJar,
    State(database): State<Database>,
    State(config): State<Config>,
    TypedHeader(_user_agent): TypedHeader<UserAgent>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Json(login_schema): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    debug!("login_schema is {:#?}", login_schema);

    let user = login_schema.verify(&database).await?.ok_or_else(|| {
        AppError::NotFoundError("User not found with this username and password".to_string())
    })?;

    let new_access = create_token(&config, user.id, false)?;
    let new_refresh = create_token(&config, user.id, true)?;

    let max_age_days = config.refresh_token_expire_in_days.unwrap_or(30);
    let refresh_cookie = Cookie::build(("refresh_token", new_refresh.clone()))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::days(max_age_days))
        .secure(config.cookie_secure.unwrap_or(true));

    let jar = jar.add(refresh_cookie);

    let tokens = Tokens {
        access_token: new_access,
        refresh_token: Some(new_refresh),
    };
    let response = Json(AuthResponse { user, tokens });
    Ok((jar, response))
}

pub async fn get_user_handler(
    claims: Claims,
    State(database): State<Database>,
) -> Result<impl IntoResponse, AppError> {
    debug!("claims: {:#?}", claims);

    let user = sqlx::query_as!(
        User,
        r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                phone_number,
                password,
                picture,
                role AS "role: UserRole",
                status AS "status: UserStatus",
                created_at,
                updated_at
            FROM users WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_optional(&database.pool)
    .await?
    .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn update_user_handler() {}

pub async fn delete_user_handler(
    claims: Claims,
    State(database): State<Database>,
) -> Result<impl IntoResponse, AppError> {
    debug!("claims: {:#?}", claims);

    let query_result = sqlx::query!("DELETE FROM users WHERE id = $1", claims.sub)
        .execute(&database.pool)
        .await?;

    match query_result.rows_affected() {
        0 => Err(AppError::NotFoundError("User not found".to_string())),
        _ => Ok(StatusCode::NO_CONTENT),
    }
}

pub async fn refresh_handler(
    State(config): State<Config>,
    jar: PrivateCookieJar,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<impl IntoResponse, AppError> {
    let (token, is_web) = if let Some(cookie) = jar.get("refresh_token") {
        (cookie.value().to_string(), true)
    } else if let Some(TypedHeader(Authorization(bearer))) = auth_header {
        (bearer.token().to_string(), false)
    } else {
        return Err(AppError::MissingAuthorizationToken);
    };

    let claims = verify_token(&config, &token)?;
    if claims.typ != TokenType::Refresh {
        return Err(AppError::Unauthorized("Refresh token required".into()));
    }

    let now = Utc::now().timestamp();
    let threshold_secs = config.refresh_token_renewal_threshold_days.unwrap_or(7) * 24 * 60 * 60;
    let new_refresh = if claims.exp.saturating_sub(now) < threshold_secs {
        Some(create_token(&config, claims.sub, true)?)
    } else {
        None
    };

    let jar = if is_web {
        if let Some(ref refresh) = new_refresh {
            let max_age_days = config.refresh_token_expire_in_days.unwrap_or(30);
            let cookie = Cookie::build(("refresh_token", refresh.clone()))
                .http_only(true)
                .same_site(SameSite::Lax)
                .max_age(CookieDuration::days(max_age_days))
                .secure(config.cookie_secure.unwrap_or(true));
            jar.add(cookie)
        } else {
            jar
        }
    } else {
        jar
    };

    let new_access = create_token(&config, claims.sub, false)?;

    let response = Json(Tokens {
        access_token: new_access,
        refresh_token: new_refresh,
    });

    Ok((jar, response))
}
