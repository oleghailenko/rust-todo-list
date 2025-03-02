use crate::settings::DBSettings;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::error::Error;
use std::sync::Arc;

pub async fn init(s: &DBSettings) -> Result<Arc<PgPool>, Box<dyn Error>> {
    let options = PgConnectOptions::new()
        .host(&s.host)
        .port(s.port)
        .username(&s.username)
        .password(s.password.as_str())
        .database(&s.db_name);
    let pool = match PgPool::connect_with(options).await {
        Ok(pool) => Ok(pool),
        Err(e) => Err(Box::new(e)),
    };
    if let Ok(db) = &pool {
        info!("Executing migrations");
        sqlx::migrate!("./migrations").run(db).await?;
    }
    
    Ok(Arc::new(pool?))
}
