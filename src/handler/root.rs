use serde::Serialize;
use utoipa::ToSchema;
use actix_web::{get, HttpResponse, Responder};

#[derive(Serialize, ToSchema)]
pub struct Ping {
  pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct Status {
  pub name: String,
  pub status: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Root API", body = Status),
    ),
)]
#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().json(Status{
        name: "AI Summarizer API".to_string(),
        status: "OK".to_string()
    })
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Ping API", body = Ping),
    ),
)]
#[get("/ping")]
pub async fn ping() -> impl Responder {
    let obj = Ping {
        message: "pong".to_string(),
    };

    HttpResponse::Ok().json(obj)
}