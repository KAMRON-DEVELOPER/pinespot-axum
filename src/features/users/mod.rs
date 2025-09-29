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
        .route("/api/v1/auth/login", post(handlers::login_handler))
        .route("/api/v1/auth/refresh", delete(handlers::refresh_handler))
        .route("/api/v1/auth/google/me", get(handlers::get_oauth_user))
        .route(
            "/api/v1/auth/google/me",
            patch(handlers::complete_profile_handler),
        )
        .route("/api/v1/auth/google", get(handlers::google_oauth_handler))
        .route(
            "/api/v1/auth/google/callback",
            get(handlers::google_oauth_callback_handler),
        )
}
