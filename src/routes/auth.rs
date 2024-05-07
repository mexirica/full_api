use actix_web::{HttpResponse, post, Responder, web};

use crate::auth::{create_access_token, create_refresh_token, JwtResponse};
use crate::auth::password::validate_credentials;
use crate::models::users::Credentials;
use crate::repository::uow::UnitOfWork;

pub mod configure {
    use actix_web::web;

    use crate::routes::auth::login;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auth")
                .service(login)
        );
    }
}

#[utoipa::path(
responses (
(status = 200, description = "Is working!"),
(status = 401, description = "Unauthorized."),
(status = 500, description = "Internal server error.")
),
security (
("bearerAuth" = [])
)
)]
#[post("/login")]
pub async fn login(uow: web::Data<UnitOfWork>, credentials: web::Json<Credentials>) -> impl Responder {
    let credentials = credentials.into_inner();

    match validate_credentials(&credentials, &uow.user.pool).await {
        Ok(_) => {
            let access_token = match create_access_token(&credentials.username).await {
                Ok(token) => token,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            let refresh_token = match create_refresh_token(&credentials.username).await {
                Ok(token) => token,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            let qry_token = refresh_token.clone();

            let result = sqlx::query!(r#"UPDATE users SET refresh_token = $1 WHERE username = $2"#, qry_token, credentials.username)
                .execute(&uow.user.pool)
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(JwtResponse {
                    access_token,
                    refresh_token,
                }),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}