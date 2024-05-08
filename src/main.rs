use actix_web::{App, get, HttpServer, Responder};
use actix_web::web::Data;
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

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
            .app_data(Data::new(uow.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone()
            ))
            .service(health)
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::supplier::configure::handler)

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