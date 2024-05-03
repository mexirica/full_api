use actix_web::{HttpResponse, post, Responder, web};
use sqlx::SqlitePool;

use crate::auth::{create_access_token, create_refresh_token, JwtResponse};
use crate::auth::password::validate_credentials;
use crate::models::users::Credentials;

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


#[post("/login")]
pub async fn login(pool: web::Data<SqlitePool>, credentials: web::Json<Credentials>) -> impl Responder {
    let credentials = credentials.into_inner();

    match validate_credentials(&credentials, &pool).await {
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
                .execute(pool.get_ref())
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
