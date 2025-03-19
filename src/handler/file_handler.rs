use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::path::Path;
use futures::StreamExt; // import undirectly for .next() in payload
use std::{
  io::Write, // import undirectly for .file.write_all()
  io::Read, // import undirectly for .file.read_to_string()
  fs::File
};
use uuid::Uuid;
use pdf_extract;
use docx_rust::*;

use crate::handler::{
  request::SummaryRequest, 
  response::{error_resp, SummarizeResponse, UploadResponse}
};
use crate::libs::openai::chat_completion;

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
pub async fn upload_file_handler(_auth: BearerAuth, mut payload: Multipart) -> impl Responder {
  // if validate_jwt(auth.token()).is_err() {
  //   return error_resp("Unauthorized", 401);
  // }

  let upload_dir = "upload";
  std::fs::create_dir_all(upload_dir).unwrap();

  let file_id = Uuid::new_v4().to_string().replace("-", "");
  let mut extension: String = String::new();

  while let Some(Ok(mut field)) = payload.next().await {
    if field.name() != Some("file") {
      return error_resp("Invalid field name", 400);
    }

    // let content_type = field.content_type().clone();
    let content_disposition = field.content_disposition().unwrap();
    let filename = content_disposition.get_filename().unwrap();

    // Get the file extension
    extension = Path::new(filename)
      .extension()
      .and_then(std::ffi::OsStr::to_str)
      .unwrap_or("").to_string();

    extension = extension.to_lowercase();

    // Check allowed extensions
    let allowed_extensions = ["pdf", "docx", "txt"];
    if !allowed_extensions.contains(&extension.as_str()) {
      return error_resp("Unsupport file type", 400);
    }

    // Generate unique filename
    let new_filename = format!("{}.{}", file_id, extension);
    let filepath = format!("./{}/{}", upload_dir, new_filename);
    // Save file

    let mut file = web::block(move || File::create(filepath.clone())).await.unwrap().unwrap();
    while let Some(Ok(chunk)) = field.next().await {
      file.write_all(&chunk).unwrap();
    }

    println!("created file: {}", new_filename);
  }

  return HttpResponse::Ok().json(UploadResponse{
    file_id: &file_id,
    file_ext: &extension,
  });
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
pub async fn summarize(_auth: BearerAuth, req: web::Json<SummaryRequest>) -> impl Responder {
  // if validate_jwt(auth.token()).is_err() {
  //   return error_resp("Unauthorized", 401);
  // }

  let file_ext = req.file_ext.clone();
  let file_name = format!("./upload/{}.{}", req.file_id, file_ext);

  let extracted_text = match file_ext.as_str() {
    "pdf" => extract_text_from_pdf(&file_name.clone()),
    "docx" => extract_text_from_docx(&file_name.clone()),
    "txt" => {
      let mut file = File::open(&file_name).unwrap();
      let mut contents = String::new();

      file.read_to_string(&mut contents).unwrap();
      contents
    }
    _ => "Unsupported file type".to_string(),
  };

  if extracted_text.clone() == "Unsupported file type" {
    return error_resp("Unsupported file type", 400);
  }

  let summary = chat_completion(&extracted_text.clone()).await;
  let summary = match summary {
    Ok(_) => summary.unwrap(),
    Err(_) => {
      return error_resp("Error summarizing text", 500);
    } 
  };

  HttpResponse::Ok().json(SummarizeResponse{
    summary: &summary,
    content: &extracted_text,
  })
}

// internal function
fn extract_text_from_pdf(file_path: &str) -> String {
  match pdf_extract::extract_text(file_path) {
    Ok(text) => text,
    Err(_) => "Error extracting text from PDF".to_string(),
  }
}

fn extract_text_from_docx(file_path: &str) -> String {
  let docx = DocxFile::from_file(file_path).unwrap();
  let docx = docx.parse().unwrap();

  let extract_data = docx.document.body.text();
  extract_data
}