use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenv::dotenv;

pub async fn connect_db() -> DatabaseConnection {
  dotenv().ok(); // Load .env file if available
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  
  Database::connect(db_url).await.expect("Failed to connect to database")
}