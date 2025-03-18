use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
  encode, decode, 
  Header, Validation, 
  EncodingKey, DecodingKey
};
use jsonwebtoken::{
  TokenData, 
  errors::Error as JwtError
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub email: String,
  pub exp: usize,
}

impl Claims {
  fn with_data(id: &str, email: &str) -> Self {
    let expiration = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs() as usize + (24 * 3600); // 24-hour expiry

    Claims {
      sub: id.to_owned(),
      email: email.to_owned(),
      exp: expiration as usize,
    }
  }
}

pub fn generate_jwt(user_id: &str, email: &str) -> String {
  dotenv().ok();
  let secret_key = env::var("SECRET_KEY").unwrap_or("".to_string());
  
  let claims = Claims::with_data(user_id, email);
  encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, JwtError> {
  dotenv().ok();
  let secret_key = env::var("SECRET_KEY").unwrap_or("".to_string());

  decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default())
}