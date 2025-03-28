mod libs;
mod docs;
mod model;
mod driver;
mod handler;
mod service;
mod middleware;

use actix_web::{web, middleware::Logger};
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use service::{file::FileService, auth::AuthService};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    let conn = Arc::new(driver::db::create_pool().await.unwrap());

    let file_service = Arc::new(FileService::new());
    let auth_service = Arc::new(AuthService::new(conn.clone()));

    println!("🚀 Server started on port :8080");

    HttpServer::new(move || {
        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
            .app_data(web::Data::new(file_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .service(handler::root::root)
            .service(handler::root::ping)
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

