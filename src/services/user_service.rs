use actix_web::{HttpRequest, HttpResponse};

use crate::auth;
use crate::auth::compute_password_hash;
use crate::db::repository::Repository;
use crate::models::users::{ChangePassword, Credentials, Users};
use crate::db::UnitOfWork;

pub(crate) async fn handle_change_password(
    uow: &UnitOfWork,
    json: &ChangePassword,
    req: &HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let password = &json.password;
    let claims = auth::get_claims(&req);

   if claims.is_err() {
       return Err(HttpResponse::InternalServerError().finish());
   }
    let claims = claims.unwrap();

    match uow.user.find_by_username(&claims.sub).await {
        Ok(None) => Err(HttpResponse::BadRequest().finish()),
        Ok(Some(_)) => {
            let password_hashed = match compute_password_hash(&password) {
                Ok(hash) => hash,
                Err(_) => return Err(HttpResponse::InternalServerError().finish()),
            };

            match uow.user.change_password( &claims.sub, &password_hashed).await {
                Ok(_) => Ok(HttpResponse::NoContent().finish()),
                Err(_) => Err(HttpResponse::InternalServerError().finish()),
            }
        }
        _ => Err(HttpResponse::InternalServerError().finish()),
    }
}


pub async fn handle_create_user(
    uow: &UnitOfWork,
    user: &Credentials,
) -> Result<HttpResponse, HttpResponse> {
    let username = &user.username;
    let password = &user.password;

    if let Ok(existing_user) = uow.user.find_by_username(username).await {
        if existing_user.is_some() {
            return Err(HttpResponse::BadRequest().finish());
        }
    } else {
        return Err(HttpResponse::InternalServerError().finish());
    }

    let password_hashed = match compute_password_hash(password) {
        Ok(hash) => hash,
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
    };

    let user = Users {
        username: username.to_string(),
        password: password_hashed.to_string(),
        refresh_token: String::new(),
    };

    match uow.user.save(user).await {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
    }
}