use crate::utilities::{config::Config, errors::AppError};
use qdrant_client::Qdrant;

pub async fn build_qdrant(config: &Config) -> Result<Qdrant, AppError> {
    let client = Qdrant::from_url(
        &config
            .qdrant_url
            .as_ref()
            .ok_or_else(|| AppError::MissingQdrantUrlError)?,
    )
    .api_key(
        config
            .qdrant_url
            .clone()
            .ok_or_else(|| AppError::MissingQdrantApiKeyError)?,
    )
    .timeout(std::time::Duration::from_secs(10))
    .build()?;

    let _health_check_reply = client.health_check().await?;

    Ok(client)
}
