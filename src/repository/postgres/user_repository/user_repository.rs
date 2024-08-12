use crate::domain::user::user::User;
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use std::sync::Arc;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn users(&self) -> Result<Vec<User>, Error>;
    async fn create(&self, user: User) -> Result<User, Error>;
}

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&*self.pool)
            .await?;

        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "
            INSERT INTO users (id, name, email)
            VALUES ($1, $2, $3)
            RETURNING id, name, email
            ",
            user.id,
            user.name,
            user.email
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
