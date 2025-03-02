use crate::model::user::{CreateUserRequest, User, UserResponse};
use crate::service::AppError;
use crate::service::Error::{AlreadyExists, InternalError, NotFound};
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

    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
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

    pub async fn get_user_by_username(&self, username: &str) -> Result<UserResponse, AppError> {
        let user = self.find_user_by_username(username).await?;
        match user {
            None => Err(AppError {
                error: NotFound,
                message: format!("User {} not found", username),
            }),
            Some(u) => {
                let stats = sqlx::query("SELECT count(*) FROM lists WHERE user_id = $1")
                    .bind(u.id)
                    .fetch_one(&*self.db_pool)
                    .await?;
                let list_count: i64 = stats.try_get(0)?;
                Ok(UserResponse {
                    id: u.id,
                    username: u.username,
                    lists: list_count as i32,
                    items: 0,
                    done: 0,
                })
            }
        }
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<i64, AppError> {
        let transaction = self.db_pool.begin().await?;

        let user = self.find_user_by_username(&request.username).await?;
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

    pub async fn user_list(&self, limit: u16, page: u32) -> Result<Vec<User>, AppError> {
        let rows = sqlx::query("SELECT id, username FROM users ORDER BY id LIMIT $1 OFFSET $2")
            .bind(limit as i32)
            .bind(((page - 1) as i32 * limit as i32) as i64)
            .fetch_all(&*self.db_pool)
            .await?;

        rows.iter()
            .map(|row| {
                let id = row.try_get("id")?;
                let username = row.try_get("username")?;
                Ok(User { id, username })
            })
            .into_iter()
            .collect()
    }
}
