mod libs;
mod model;
mod driver;
mod handler;
mod middleware;

use actix_web::{web, middleware::Logger};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use handler::request::{LoginRequest, RegisterRequest, SummaryRequest};
use handler::response::{LoginResponse, RegisterResponse, UploadResponse, SummarizeResponse, Error};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        root,
        ping,
        handler::auth::login, 
        handler::auth::register, 
        handler::file_handler::upload_file_handler, 
        handler::file_handler::summarize
    ), 
    components(schemas(Ping, Status, LoginRequest, RegisterRequest, SummaryRequest, RegisterResponse, LoginResponse, UploadResponse, SummarizeResponse, Error))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    let db = driver::db::connect_db().await;
    let db_data = web::Data::new(db);

    println!("🚀 Server started on port :8080");

    HttpServer::new(move || {
        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            .service(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi.clone()))
            .app_data(db_data.clone())
            .service(root)
            .service(ping)
            .service(handler::auth::login)
            .service(handler::auth::register)
            .service(
                web::scope("/file")
                .wrap(HttpAuthentication::bearer(middleware::validator))
                .route("/upload", web::post().to(handler::file_handler::upload_file_handler))
                .route("/summarize", web::post().to(handler::file_handler::summarize)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize, ToSchema)]
struct Ping {
    message: String,
}

#[derive(Serialize, ToSchema)]
struct Status {
    name: String,
    status: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Root API", body = Status),
    ),
)]
#[get("/")]
async fn root() -> impl Responder {
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
async fn ping() -> impl Responder {
    let obj = Ping {
        message: "pong".to_string(),
    };

    HttpResponse::Ok().json(obj)
}