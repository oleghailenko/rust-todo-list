use sqlx::{PgPool, Row};
use std::sync::Arc;
use crate::model::item::{CreateItemRequest, Item};
use crate::service::AppError;

pub struct ItemService {
    db_pool: Arc<PgPool>,
}

impl ItemService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn list_items(
        &self,
        user_id: i64,
        list_id: i64,
        limit: i32,
        page: i32
    ) -> Result<Vec<Item>, AppError> {
        let rows = sqlx::query("SELECT id, description, done FROM items 
                             WHERE user_id = $1 AND list_id = $2
                             ORDER BY id LIMIT $3 OFFSET $4")
            .bind(user_id)
            .bind(list_id)
            .bind(limit)
            .bind((page - 1) * limit)
            .fetch_all(&*self.db_pool)
            .await?;

        rows.iter()
            .map(|row| {
                let id: i64 = row.try_get("id")?;
                let description: String = row.try_get("description")?;
                let done: bool = row.try_get("done")?;

                Ok(Item { id, description, done })
            })
            .into_iter()
            .collect()
    }

    pub async fn create_item(&self, user_id: i64, list_id: i64, request: &CreateItemRequest) -> Result<i64, AppError> {
        let result = sqlx::query("INSERT INTO items (user_id, list_id, description, done) 
                    VALUES ($1, $2, $3, 'f') RETURNING id")
            .bind(user_id)
            .bind(list_id)
            .bind(&request.description)
            .fetch_one(&*self.db_pool).await?;
        let id: i64 = result.try_get("id")?;
        Ok(id)
    }
}
