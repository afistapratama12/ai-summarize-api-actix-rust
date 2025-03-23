use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Users {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct Login {
    pub name: String,
    pub token: String,
}