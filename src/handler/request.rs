use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Clone, Deserialize, ToSchema)]
pub struct RegisterRequest {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, ToSchema)]
pub struct SummaryRequest {
  pub files: Vec<FileSummary>,
  pub lang: String,
  pub input_text: Option<String>,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct FileSummary {
  pub file_id: String,
  pub file_ext: String,
  pub file_name_upload: String,
}

// #[derive(Clone, Deserialize)]
// pub struct SummarizationRequest {
//   pub model: String,
//   pub prompt: String,
//   pub max_tokens: usize,
// }