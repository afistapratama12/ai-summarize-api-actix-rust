use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::path::Path;
use futures::StreamExt; // import undirectly for .next() in payload
use uuid::Uuid;
use std::sync::Arc;

use crate::{
  handler::{
    request::SummaryRequest, 
    response::{error_resp, SummarizeResponse, UploadResponse}
  }, 
  service::file::FileService
};

#[utoipa::path(
  post,
  path = "/file/upload",
  security(("bearerAuth" = [])),
  request_body(content_type = "multipart/form-data", content = String, description = "File to upload"),
  responses(
    (status = 200, description = "upload successfully", body = UploadResponse),
    (status = BAD_REQUEST, description = "Bad request", body = Error),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error),
  ),
)]
pub async fn upload_file_handler(_auth: BearerAuth, service: web::Data<Arc<FileService>>, mut payload: Multipart) -> impl Responder {
  let upload_dir = "upload";

  // if directory not exist, create directory
  if !Path::new(upload_dir).exists() {
    std::fs::create_dir_all(upload_dir).unwrap();
  }

  while let Some(field) = payload.next().await {
    match field {
      Ok(field) => {
        let content_disposition = field.content_disposition().clone().unwrap();
        let filename = content_disposition
          .get_filename()
          .map(String::from)
          .unwrap_or_else(|| "".to_string());

        let extension = filename.split('.').last().unwrap_or("");
        let extension = extension.to_lowercase();
        if !["pdf", "docx", "txt"].contains(&extension.as_str()) {
          return error_resp("unsupport file extention, only allow (pdf, docx, txt)", 400)
        }

        // Generate unique filename
        let file_id = Uuid::new_v4().to_string().replace("-", "");
        let filepath = format!("./{}/{}.{}", upload_dir, file_id, extension);

        let resp = service.upload_file(filepath, field).await; 
        match resp {
          Ok(_) => (),
          Err(err) => return error_resp(&format!("{}", err), 500)
        }

        return HttpResponse::Ok().json(UploadResponse{
          file_id: &file_id,
          file_ext: extension.as_str(),
          file_name_upload: &filename,
        });
      }
      Err(e) => return error_resp(&format!("Error reading file: {}", e), 500),
    }
  }

  error_resp("invalid file upload", 400)
}

#[utoipa::path(
  post,
  path = "/file/summarize",
  security(("bearerAuth" = [])),
  request_body = SummaryRequest,
  responses(
    (status = 200, description = "file summarize successfully", body = SummarizeResponse),
    (status = BAD_REQUEST, description = "Bad request", body = Error),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error),
  ),
)]
pub async fn summarize(_auth: BearerAuth, service: web::Data<Arc<FileService>>, req: web::Json<SummaryRequest>) -> impl Responder {
  if req.files.len() == 0 {
    return error_resp("no file to summarize", 400)
  }

  if req.lang != "id" && req.lang != "en" {
    return error_resp("unsupported language, only allow language \"id\" and \"en\"", 400)
  }

  let summary = service.summarize(req.files.clone(), req.lang.clone(), req.input_text.clone()).await;
  let summary = match summary {
    Ok(output) => output,
    Err(err) => return error_resp(&format!("{}", err), 500)
  };

  HttpResponse::Ok().json(SummarizeResponse{
    summary: &summary,
  })
}
