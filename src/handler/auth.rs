use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use futures::TryStreamExt as _;
use chrono;

use crate::{
  model::users::Users,
  libs::{hash, jwt},
  handler::{  
    request::{RegisterRequest, LoginRequest},
    response::{error_resp, LoginResponse, RegisterResponse}
  },
};

#[utoipa::path(
  post,
  path = "/auth/login",
  request_body = LoginRequest,
  responses(
      (status = 200, description = "login successfully", body = LoginResponse),
      (status = BAD_REQUEST, description = "Bad request", body = Error),
      (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error),
  ),
)]
#[post("/auth/login")]
pub async fn login(db: web::Data<Pool<Postgres>>, req: web::Json<LoginRequest>) -> impl Responder {
  if !req.email.contains("@") { 
    error_resp("invalid email format", 400); 
  }

  let mut stream = sqlx::query_as::<_, Users>("SELECT * FROM users WHERE email = $1")
    .bind(req.email.clone())
    .fetch(db.as_ref());

  let user = match stream.try_next().await {
    Ok(Some(user)) => user,
    Ok(None) => return error_resp("error find user", 500),
    Err(e) => {
      eprintln!("Error finding user: {:?}", e);
      return error_resp("error find user", 500);
    }
  };

  // check password
  if !hash::verify_password(&req.password, &user.password) {
    return error_resp("invalid email or password", 400);
  }

  let token = jwt::generate_jwt(user.id.as_str(), &user.email);

  HttpResponse::Ok().json(LoginResponse{
    name: &user.name,
    email: &user.email,
    token: &token,
  })
}

#[utoipa::path(
  post,
  path = "/auth/register",
  request_body = RegisterRequest,
  responses(
      (status = 201, description = "register successfully", body = RegisterResponse),
      (status = BAD_REQUEST, description = "Bad request", body = Error),
      (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error),
  ),
  // params(
  // ("id" = u64, Path, description = "Pet database id to get Pet for"),
  //)
)]
#[post("/auth/register")]
pub async fn register(db: web::Data<Pool<Postgres>>, req: web::Json<RegisterRequest>) -> impl Responder {
  if !req.email.contains("@") {
    return error_resp("invalid email format", 400)
  }

  let stream = sqlx::query("INSERT INTO users (id, name, email, password, created_at) VALUES ($1, $2, $3, $4, $5)")
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(req.name.clone())
    .bind(req.email.clone())
    .bind( hash::hash_password(req.password.clone().as_str()))
    .bind(chrono::Utc::now())
    .execute(db.as_ref());

  match stream.await {
    Ok(_) => {},
    Err(e) => {
      eprintln!("Error inserting new user: {:?}", e);
      // get error message from sqlx
      let error_message = e.to_string();
      if error_message.contains("duplicate key value violates unique constraint") {
        return error_resp("email already registered", 400);
      } else {
        return error_resp("error inserting new user", 500);
      }
    }
  }

  HttpResponse::Created().json(RegisterResponse{
    name: &req.name,
    email: &req.email,
    message: "user registered successfully",
  })
}