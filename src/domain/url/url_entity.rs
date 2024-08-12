use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Url {
    pub id: Uuid,
    pub url_full: String,
    pub url_short: String,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
