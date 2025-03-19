use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

pub async fn create_pool() -> Result<sqlx::PgPool, sqlx::Error> {
  dotenv().ok(); // Load .env file if available
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
}