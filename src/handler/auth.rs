use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::{
  handler::{  
    request::{LoginRequest, RegisterRequest},
    response::{error_resp, LoginResponse, RegisterResponse}
  }, 
  service::auth::AuthService
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
pub async fn login(service: web::Data<Arc<AuthService>>, req: web::Json<LoginRequest>) -> impl Responder {
  if !req.email.contains("@") { 
    error_resp("invalid email format", 400); 
  }

  let res = service.login(&req.email, &req.password).await;
  let res = match res {
    Ok(res) => res,
    Err(e) => return error_resp(&e, 400),
  };

  HttpResponse::Ok().json(LoginResponse{
    name: &res.name,
    email: &req.email,
    token: &res.token,
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
pub async fn register(service: web::Data<Arc<AuthService>>, req: web::Json<RegisterRequest>) -> impl Responder {
  if !req.email.contains("@") {
    return error_resp("invalid email format", 400)
  }

  let res = service.register(&req.name, &req.email, &req.password).await;
  let _res = match res {
    Ok(res) => res,
    Err(e) => {
      if e.contains("email already exist") {
        return error_resp(e.as_str(), 400);
      } else {
        return error_resp(e.as_str(), 500);
      }
    }
  };

  HttpResponse::Created().json(RegisterResponse {
    name: &req.name,
    email: &req.email,
    message: "user registered successfully",
  })
}