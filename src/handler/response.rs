use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse<'a> {
  pub name: &'a str,
  pub email: &'a str,
  pub message: &'a str,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse<'a> {
  pub name: &'a str,
  pub email: &'a str,
  pub token: &'a str,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResponse<'a> {
  pub file_id: &'a str,
  pub file_ext: &'a str,
  // pub extracted_text: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SummarizeResponse<'a> {
  pub summary: &'a str,
  pub content: &'a str,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Error<'a> {
  pub message: &'a str,
  pub status: u16,
}

pub fn error_resp(msg: &str, status: u16) -> HttpResponse {
  match status {
    400 => HttpResponse::BadRequest().json(Error {
      message: msg,
      status,
    }),
    401 => HttpResponse::Unauthorized().json(Error {
      message: msg,
      status,
    }),
    403 => HttpResponse::Forbidden().json(Error {
      message: msg,
      status,
    }),
    404 => HttpResponse::NotFound().json(Error {
      message: msg,
      status,
    }),
    500 => HttpResponse::InternalServerError().json(Error {
      message: msg,
      status,
    }),
    _ => HttpResponse::BadRequest().json(Error {
      message: msg,
      status,
    }),      
  }
}