use actix_web::{dev::ServiceRequest, Error, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::libs::jwt::validate_jwt;

// Function to validate JWT and handle errors
pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let token = credentials.token();
  match validate_jwt(token) {
    Ok(_) => Ok(req),
    // Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid or expired token")),
    Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid or expired token"), req)),
  }
}