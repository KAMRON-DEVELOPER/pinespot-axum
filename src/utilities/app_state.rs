use crate::{
    services::{ai::AI, database::Database, redis::Redis},
    utilities::{
        config::Config,
        google_oauth_openidconnect::GoogleOAuthOpenIdConnectClient,
        oauth_client_builder::{GithubOAuthClient, GoogleOAuthClient},
    },
};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use object_store::{aws::AmazonS3, gcp::GoogleCloudStorage};
use qdrant_client::Qdrant;
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub redis: Redis,
    pub qdrant: Qdrant,
    pub ai: AI,
    pub config: Config,
    pub key: Key,
    pub google_oauth_client: GoogleOAuthClient,
    pub github_oauth_client: GithubOAuthClient,
    pub oauth_openidconnect_client: GoogleOAuthOpenIdConnectClient,
    pub http_client: Client,
    pub s3: AmazonS3,
    pub gcs: GoogleCloudStorage,
}

impl FromRef<AppState> for Database {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

impl FromRef<AppState> for Redis {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}

impl FromRef<AppState> for Qdrant {
    fn from_ref(state: &AppState) -> Self {
        state.qdrant.clone()
    }
}

impl FromRef<AppState> for AI {
    fn from_ref(state: &AppState) -> Self {
        state.ai.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for GoogleOAuthClient {
    fn from_ref(state: &AppState) -> Self {
        state.google_oauth_client.clone()
    }
}

impl FromRef<AppState> for GithubOAuthClient {
    fn from_ref(state: &AppState) -> Self {
        state.github_oauth_client.clone()
    }
}

impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.http_client.clone()
    }
}

impl FromRef<AppState> for AmazonS3 {
    fn from_ref(state: &AppState) -> Self {
        state.s3.clone()
    }
}

impl FromRef<AppState> for GoogleCloudStorage {
    fn from_ref(state: &AppState) -> Self {
        state.gcs.clone()
    }
}
