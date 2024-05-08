use actix_web::{App, get, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::auth::handle_unauthorized;

mod auth;
mod models;
mod routes;
mod services;
mod docs;
mod log;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    log4rs::init_config(log::configure_log()).unwrap();
    let openapi = docs::ApiDoc::openapi();

    let pool = db::connect().await;
    let uow = db::UnitOfWork::new(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(uow.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone()
            ))
            .service(health)
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::supplier::configure::handler)
            .default_service(web::route().to(handle_unauthorized))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
#[utoipa::path(
responses (
(status = 200, description = "Is working!"),
(status = 500, description = "Internal server error.")
))]
#[get("/health")]
pub async fn health() -> impl Responder {
    "Is working!"
}