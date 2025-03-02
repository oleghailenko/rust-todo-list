use crate::model::list::{CreateListRequest, ListResponse};
use crate::service::AppError;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct ListService {
    db_pool: Arc<PgPool>,
}
 impl ListService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        ListService { db_pool }
    }

    pub async fn create_list(&self, user_id: i64, request: &CreateListRequest) -> Result<i64, AppError> {
        let result = sqlx::query("INSERT INTO lists (name, user_id) VALUES ($1, $2) RETURNING id")
            .bind(&request.name)
            .bind(user_id)
            .fetch_one(&*self.db_pool)
            .await?;
        let id: i64 = result.try_get("id")?;
        Ok(id)
    }

    pub async fn get_lists(&self, user_id: i64, limit: i32, page: i32) -> Result<Vec<ListResponse>, AppError> {
        let rows = sqlx::query("SELECT id, name FROM lists WHERE user_id = $1 ORDER BY id LIMIT $2 OFFSET $3")
            .bind(user_id)
            .bind(limit)
            .bind((page - 1) * limit)
            .fetch_all(&*self.db_pool)
            .await?;
        rows.iter()
            .map(|row| {
                let id: i64 = row.try_get("id")?;
                let name = row.try_get("name")?;
                Ok(ListResponse {
                    id,
                    name,
                    total_items: 0,
                    done_items: 0,
                })
            })
            .into_iter()
            .collect()
    }
     
     pub async fn get_list(&self, user_id: i64, list_id: i64) -> Result<ListResponse, AppError> {
         let row = sqlx::query("SELECT id, name FROM lists WHERE user_id = $1 AND id = $2")
             .bind(user_id)
             .bind(list_id)
             .fetch_one(&*self.db_pool).await?;
         let id = row.try_get("id")?;
         let name = row.try_get("name")?;
         
         Ok(ListResponse{
             id,
             name,
             total_items: 0,
             done_items: 0
         })
     }
}
