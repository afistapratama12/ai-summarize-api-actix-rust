use utoipa::OpenApi;

use crate::handler::request::{LoginRequest, RegisterRequest, SummaryRequest};
use crate::handler::response::{LoginResponse, RegisterResponse, UploadResponse, SummarizeResponse, Error};
use crate::handler::root::{Ping, Status};

#[derive(OpenApi)]
#[openapi(
  paths(
    crate::handler::root::root,
    crate::handler::root::ping,
    crate::handler::auth::login, 
    crate::handler::auth::register, 
    crate::handler::file_handler::upload_file_handler, 
    crate::handler::file_handler::summarize
  ), 
  components(
    schemas(Ping, Status, LoginRequest, RegisterRequest, SummaryRequest, RegisterResponse, LoginResponse, UploadResponse, SummarizeResponse, Error)
  )
)]
pub struct ApiDoc;


