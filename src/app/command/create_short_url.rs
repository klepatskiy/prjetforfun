use async_trait::async_trait;
use chrono::{Utc};
use uuid::Uuid;
use crate::app::error::AppError;
use crate::domain::url::url::Url;

#[mockall::automock]
#[async_trait]
pub trait CreateShortUrlRepository {
    async fn create(&self, url: Url) -> Result<String, AppError>;
}

pub struct CreateShortUrlCommand<R>
where
    R: CreateShortUrlRepository,
{
    repo: R,
}

impl<R> CreateShortUrlCommand<R>
where
    R: CreateShortUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, full_url: String) -> Result<String, AppError> {
        let parsed_url = url::Url::parse(&*full_url.clone()).map_err(|e| AppError::URLParseError)?;
        let new_url = Url {
            id: Uuid::new_v4(),
            url_full: full_url.to_string(),
            url_short: parsed_url.to_string(),
            user_id: None,
            created_at: Utc::now(),
        };
        
        self.repo.create(new_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        let mut mock_repo = MockCreateShortUrlRepository::new();
        mock_repo.expect_create().returning(|_| Ok("short.ly/abc123".to_string())).times(1);
        let command = CreateShortUrlCommand::new(mock_repo);
        let result = command.execute("https://www.google.com".to_string()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "short.ly/abc123".to_string());
    }
}
