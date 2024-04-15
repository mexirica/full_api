use actix_web::{delete, HttpRequest, HttpResponse, post, put, Responder, web};
use sqlx::SqlitePool;

use crate::models::users::*;
use crate::repository::Repository;
use crate::services::user_service::{handle_change_password, handle_create_user};

pub mod configure {
    use actix_web::web;

    use crate::routes::users::*;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/users")
                .service(create_user)
                .service(delete_by_username)
                .service(change_password),
        );
    }
}

#[post("")]
pub async fn create_user(pool: web::Data<SqlitePool>, user: web::Json<Credentials>) -> impl Responder {
    handle_create_user(&pool, &user.into_inner()).await
        .unwrap_or_else(|error| error)
}

#[delete("")]
pub async fn delete_by_username(
    pool: web::Data<SqlitePool>,
    username: web::Query<UsernameQuery>,
) -> impl Responder {
    match Users::delete_by_username(&pool, username.into_inner().username).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("")]
pub async fn change_password(
    pool: web::Data<SqlitePool>,
    json: web::Json<ChangePassword>,
    req: HttpRequest,
) -> impl Responder {
    handle_change_password(&pool, &json.into_inner(), &req)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap_or_else(|error| error)
}
