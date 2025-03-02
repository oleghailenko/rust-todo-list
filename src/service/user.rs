use crate::model::user::{CreateUserRequest, User};
use crate::service::AppError;
use crate::service::Error::{AlreadyExists, InternalError};
use rocket::futures::TryStreamExt;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct UserService {
    db_pool: Arc<PgPool>,
}

impl UserService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let mut rows = sqlx::query("SELECT id, username FROM users WHERE username = $1")
            .bind(username)
            .fetch(&*self.db_pool);

        let result = rows.try_next().await?;
        match result {
            Some(row) => {
                let id: i64 = row.try_get(0)?;
                let username: String = row.try_get(1)?;
                Ok(Some(User { id, username }))
            }
            None => Ok(None),
        }
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<i64, AppError> {
        let transaction = self.db_pool.begin().await?;

        let user = self.get_user_by_username(&request.username).await?;
        if let Some(_) = user {
            return Err(AppError {
                error: AlreadyExists,
                message: format!("User '{}' already exists", &request.username),
            });
        }

        let result = sqlx::query(r#"INSERT INTO users (username) VALUES ($1) RETURNING id"#)
            .bind(&request.username)
            .fetch_one(&*self.db_pool)
            .await?;

        let result = match result.try_get(0) {
            Ok(id) => Ok(id),
            Err(e) => Err(AppError {
                error: InternalError,
                message: e.to_string(),
            }),
        };
        
        transaction.commit().await?;
        
        result
    }
}
