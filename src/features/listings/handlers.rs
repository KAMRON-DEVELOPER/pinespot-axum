use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use axum_extra::{either::Either, extract::PrivateCookieJar};
use uuid::Uuid;

use crate::{
    features::{
        listings::{models::Listing, schemas::ListingResponse},
        schemas::Pagination,
    },
    services::database::Database,
    utilities::{config::Config, cookie::OptionalOAuthUserIdCookie, errors::AppError, jwt::Claims},
};

pub async fn get_many_listings_handler(
    jar: PrivateCookieJar,
    State(database): State<Database>,
    State(config): State<Config>,
    Query(pagination): Query<Pagination>,
    OptionalOAuthUserIdCookie(optional_oauth_user_id_cookie): OptionalOAuthUserIdCookie,
) -> Result<impl IntoResponse, AppError> {
    if optional_oauth_user_id_cookie.is_some() {
        let uri = format!("{}/complete-profile", config.frontend_endpoint);
        return Ok(Either::E1((jar, Redirect::to(&uri))));
    }

    pagination.validate()?;

    let listings = sqlx::query_as!(
        Listing,
        r#"
            SELECT * FROM listings
            ORDER BY updated_at DESC
            OFFSET $1 LIMIT $2
        "#,
        pagination.offset,
        pagination.limit
    )
    .fetch_all(&database.pool)
    .await?;

    let total = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) from listings
        "#
    )
    .fetch_one(&database.pool)
    .await?;

    let total = total.unwrap_or(0);

    Ok(Either::E2(Json(ListingResponse { listings, total })))
}

pub async fn get_one_listing_handler(
    State(database): State<Database>,
    Path(listing_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let listing = sqlx::query_as!(
        Listing,
        r#"
            SELECT * FROM listings where id = $1
        "#,
        listing_id
    )
    .fetch_one(&database.pool)
    .await?;

    Ok(Json(listing))
}

pub async fn delete_listing_handler(
    Path(listing_id): Path<Uuid>,
    State(database): State<Database>,
    claims: Claims,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query_scalar!(
        r#"
            DELETE FROM listings where owner_id = $1 AND id = $2
        "#,
        claims.sub,
        listing_id
    )
    .execute(&database.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
