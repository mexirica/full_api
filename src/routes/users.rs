use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web, web::Json};

use crate::models::users::*;
use crate::repository::Repository;
use crate::repository::uow::UnitOfWork;
use crate::services::user_service::{handle_change_password, handle_create_user};
use utoipa::openapi::{self, OpenApi}; // Add this import statement

pub mod configure {
    use actix_web::web;

    use crate::routes::users::*;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/users")
                .service(create_user)
                .service(delete_by_username)
                .service(change_password)
                .service(get_user)
        );
    }
}

#[post("")]
pub async fn create_user(uow: web::Data<UnitOfWork>, user: Json<Credentials>) -> impl Responder {
    handle_create_user(&uow, &user.into_inner()).await
        .unwrap_or_else(|error| error)
}

#[delete("")]
pub async fn delete_by_username(
    uow: web::Data<UnitOfWork>,
    username: web::Query<UsernameQuery>,
) -> impl Responder {
    match uow.user.delete_by_username(username.into_inner().username).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("")]
pub async fn change_password(
    uow: web::Data<UnitOfWork>,
    json: Json<ChangePassword>,
    req: HttpRequest,
) -> impl Responder {
    handle_change_password(&uow, &json.into_inner(), &req)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap_or_else(|error| error)
}

#[utoipa::path(get, path = "/users/{username}")]
#[get("/{username}")]
pub async fn get_user(uow: web::Data<UnitOfWork>, username: web::Path<String>) -> impl Responder {
    let username = username.into_inner();


    return uow.user.find_by_username(&username)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .map_or(HttpResponse::NotFound().finish(), |user| HttpResponse::Ok().json(user))
}