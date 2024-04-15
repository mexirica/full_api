use actix_web::{HttpRequest, HttpResponse};
use sqlx::SqlitePool;

use crate::auth;
use crate::auth::compute_password_hash;
use crate::models::users::{ChangePassword, Credentials, Users};
use crate::repository::Repository;

pub(crate) async fn handle_change_password(
    pool: &SqlitePool,
    json: &ChangePassword,
    req: &HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let password = &json.password;
    let claims = auth::get_claims(&req);

   if claims.is_err() {
       return Err(HttpResponse::InternalServerError().finish());
   }
    let claims = claims.unwrap();

    match Users::find_by_username(pool, &claims.sub).await {
        Ok(None) => Err(HttpResponse::BadRequest().finish()),
        Ok(Some(_)) => {
            let password_hashed = match compute_password_hash(&password) {
                Ok(hash) => hash,
                Err(_) => return Err(HttpResponse::InternalServerError().finish()),
            };

            match Users::change_password(pool, &claims.sub, &password_hashed).await {
                Ok(_) => Ok(HttpResponse::NoContent().finish()),
                Err(_) => Err(HttpResponse::InternalServerError().finish()),
            }
        }
        _ => Err(HttpResponse::InternalServerError().finish()),
    }
}


pub async fn handle_create_user(
    pool: &SqlitePool,
    user: &Credentials,
) -> Result<HttpResponse, HttpResponse> {
    let username = &user.username;
    let password = &user.password;

    if let Ok(existing_user) = Users::find_by_username(pool, username).await {
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

    match Users::save(pool, user).await {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
    }
}