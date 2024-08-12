use crate::app::error::AppError;
use crate::domain::url::url_entity::Url;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresUrlRepository {
    pool: Arc<PgPool>,
}

impl PostgresUrlRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl crate::app::command::create_short_url::CreateShortUrlRepository for PostgresUrlRepository {
    async fn create(&self, url: Url) -> Result<String, AppError> {
        let query = "
            INSERT INTO urls (id, url_full, url_short, user_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
        ";

        sqlx::query(query)
            .bind(url.id)
            .bind(url.url_full)
            .bind(&url.url_short)
            .bind(url.user_id)
            .bind(url.created_at)
            .execute(&*self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(url.url_short)
    }
}

#[async_trait]
impl crate::app::query::get_full_url::GetFullUrlRepository for PostgresUrlRepository {
    async fn get_full_url(&self, short_url: String) -> Result<Url, AppError> {
        let query = "
            SELECT id, url_full, url_short, user_id, created_at
            FROM urls
            WHERE url_short = $1
            AND user_id IS NULL
        ";

        let url = sqlx::query_as::<_, Url>(query)
            .bind(short_url)
            .fetch_one(&*self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(url)
    }
}
