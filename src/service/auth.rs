use sqlx::{Pool, Postgres};
use std::sync::Arc;
use futures::TryStreamExt as _;

use crate::{
  libs::{hash, jwt}, 
  model::users::{Login, Users}
};

pub struct AuthService {
  pub db: Arc<Pool<Postgres>>
}

impl AuthService {
  pub fn new(db: Arc<Pool<Postgres>>) -> Self {
    Self { db }
  }

  pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<(), String> {
    let stream = sqlx::query(r#"
        INSERT INTO users (id, name, email, password, created_at) 
        VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(name)
        .bind(email)
        .bind(hash::hash_password(password))
        .bind(chrono::Utc::now())
        .execute(self.db.as_ref());

    match stream.await {
      Ok(_) => Ok(()),
      Err(e) => {
        eprintln!("Error inserting new user: {:?}", e);
        if e.to_string().contains("duplicate key value violates unique constraint") {
          return Err("email already exist".to_string());
        } else {
          return Err("error inserting new user".to_string());
        }
      }
    }
  }

  pub async fn login(&self, email: &str, password: &str) -> Result<Login, String> {
    let mut stream = sqlx::query_as::<_, Users>(r#"
      SELECT id, name, email, password, created_at::TIMESTAMPTZ, updated_at::TIMESTAMPTZ 
      FROM users 
      WHERE email = $1
      "#)
      .bind(&email)
      .fetch(self.db.as_ref());

    let user = match stream.try_next().await {
      Ok(Some(user)) => user,
      Ok(None) => return Err("error find user".to_string()),
      Err(e) => {
        eprintln!("Error finding user: {:?}", e);
        return Err("error find user".to_string());
      }
    };

    // check password
    if !hash::verify_password(password, &user.password) {
      return Err("invalid email or password".to_string());
    }

    let token = jwt::generate_jwt(user.id.as_str(), email);

    Ok(Login { 
      name: user.name.clone(), 
      token 
    })
  }


}