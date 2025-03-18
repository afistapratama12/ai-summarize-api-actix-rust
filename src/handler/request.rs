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
  pub file_id: String,
  pub file_ext: String,
  pub input_text: Option<String>,
}

// #[derive(Clone, Deserialize)]
// pub struct SummarizationRequest {
//   pub model: String,
//   pub prompt: String,
//   pub max_tokens: usize,
// }