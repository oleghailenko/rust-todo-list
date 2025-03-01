use crate::settings::DBSettings;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::error::Error;

pub async fn init(s: &DBSettings) -> Result<PgPool, Box<dyn Error>> {
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
        sqlx::migrate!("./migrations").run(db).await?;
    }
    
    Ok(pool?)
}
