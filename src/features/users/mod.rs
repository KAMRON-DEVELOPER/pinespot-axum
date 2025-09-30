pub mod handlers;
pub mod models;
pub mod schemas;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::utilities::app_state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/profile", get(handlers::get_user_handler))
        .route("/api/v1/profile", patch(handlers::update_user_handler))
        .route("/api/v1/profile", delete(handlers::delete_user_handler))
        .route("/api/v1/auth/signin", post(handlers::signin_handler))
        .route("/api/v1/auth/signup", delete(handlers::signup_handler))
        .route("/api/v1/auth/refresh", delete(handlers::refresh_handler))
        .route(
            "/api/v1/auth/google",
            patch(handlers::complete_profile_handler),
        )
        .route("/api/v1/auth/google", get(handlers::google_oauth_handler))
        .route(
            "/api/v1/auth/google/callback",
            get(handlers::google_oauth_callback_handler),
        )
        .route(
            "/api/v1/auth/google/me",
            get(handlers::get_google_oauth_user_handler),
        )
        .route("/api/v1/auth/github", get(handlers::github_oauth_handler))
        .route(
            "/api/v1/auth/github/callback",
            get(handlers::github_oauth_callback_handler),
        )
        .route(
            "/api/v1/auth/github/me",
            get(handlers::get_github_oauth_user_handler),
        )
}
