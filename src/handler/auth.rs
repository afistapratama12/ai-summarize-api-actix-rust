use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::handler::{
  request::{RegisterRequest, LoginRequest},
  response::{error_resp, LoginResponse, RegisterResponse}
};
use crate::libs::{hash, jwt};
use crate::model::{self, users::{ActiveModel, Entity as User}};

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
pub async fn login(db: web::Data<DatabaseConnection>, req: web::Json<LoginRequest>) -> impl Responder {
  if !req.email.contains("@") { 
    error_resp("invalid email format", 400); 
  }
  
  let user = User::find()
    .filter(model::users::Column::Email.contains(req.email.clone()))
    .one(db.as_ref())
    .await.unwrap();

  let user = match user {
    Some(user) => user,
    None => return error_resp("error find user", 500),
  };

  if user.email.is_empty() || user.password.is_empty() {
    return error_resp("invalid email or password", 400);
  }

  // check password
  if !hash::verify_password(req.password.as_str(), user.password.as_str()) {
    return error_resp("invalid email or password", 400);
  }

  let token = jwt::generate_jwt(user.id.as_str(), &user.email);

  HttpResponse::Ok().json(LoginResponse{
    name: user.name,
    email: user.email,
    token,
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
pub async fn register(db: web::Data<DatabaseConnection>, req: web::Json<RegisterRequest>) -> impl Responder {
  if !req.email.contains("@") {
    return error_resp("invalid email format", 400)
  }

  let new_id = uuid::Uuid::new_v4().to_string();
  let date_now = chrono::Utc::now().naive_utc();

  let hash_password = hash::hash_password(req.password.clone().as_str());

  let new_user = ActiveModel {
    id: Set(new_id.clone()),
    name: Set(req.name.clone()),
    email: Set(req.email.clone()),
    password: Set(hash_password),
    created_at: Set(date_now.clone()),
    ..Default::default()
  };

  new_user.insert(db.as_ref()).await.map_err(|e| {
    eprintln!("Error inserting new user: {:?}", e);
    error_resp("error inserting new user", 500)
  }).unwrap();


  HttpResponse::Created().json(RegisterResponse{
    name: req.name.clone(),
    email: req.email.clone(),
    message: "user registered successfully".to_string(),
  })
}