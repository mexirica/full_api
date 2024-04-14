use actix_web::{delete, get, HttpResponse, post, put, Responder, web};
use sqlx::SqlitePool;

use crate::auth;
use crate::auth::{compute_password_hash, JwtResponse};
use crate::models::users::*;
use crate::repository::Repository;

pub mod configure {
    use actix_web::web;

    use crate::routes::users::*;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/users")
                .service(create_user)
                .service(get_by_username)
                .service(delete_by_username)
                .service(get_all)
                .service(change_password)
        );
    }
}


#[post("")]
pub async fn create_user(pool: web::Data<SqlitePool>, user: web::Json<NewUser>) -> impl Responder {
    let user = user.into_inner();
    let username = &user.username;
    let password = &user.password;


    if let Ok(existing_user) = Users::find_by_username(pool.get_ref(),username).await
    {
        if existing_user.is_some() {
            return HttpResponse::BadRequest().finish();
        }
    } else {
        return HttpResponse::InternalServerError().finish();
    }

    let password_hashed = match compute_password_hash(password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let access_token = match auth::create_access_token(username).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let refresh_token = match auth::create_refresh_token(username).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let user = Users{username:username.to_string(),password:password_hashed.to_string(),refresh_token:refresh_token.to_string()};

    match Users::save(pool.get_ref(), user).await
    {
        Ok(_) => {
            HttpResponse::Created().json(JwtResponse {
                access_token,
                refresh_token,
            })
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("")]
pub async fn get_by_username(pool: web::Data<SqlitePool>, username: web::Query<UsernameQuery>) -> impl Responder{
    match Users::find_by_username(pool.get_ref(),&username.into_inner().username).await{
        Ok(user) => {
           if let Some(x) = user{
              return HttpResponse::Ok().json(x)
           }
            return HttpResponse::NotFound().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("")]
pub async fn delete_by_username(pool: web::Data<SqlitePool>, username: web::Query<UsernameQuery>)-> impl Responder{
    match Users::delete_by_username(pool.get_ref(),username.into_inner().username).await {
        Ok(_) => {HttpResponse::NoContent().finish()}
        Err(_) => {HttpResponse::InternalServerError().finish()}
    }
}

#[get("/all")]
pub async fn get_all(pool: web::Data<SqlitePool>) -> impl Responder{
    return match Users::find_all(pool.get_ref()).await {
        Ok(users) => { HttpResponse::Ok().json(users)}
        Err(_) => {HttpResponse::InternalServerError().finish()}
    }
}

#[put("/{username}")]
pub async fn change_password(pool: web::Data<SqlitePool>, json: web::Json<ChangePassword>,path: web::Path<String>) -> impl Responder{
    let password = json.into_inner().password;
    let username = path.into_inner();

    match Users::find_by_username(pool.get_ref(),&username).await {
        Ok(None) => {return HttpResponse::BadRequest().finish()}
        Ok(Some(_)) => {
            let password_hashed = match compute_password_hash(&password) {
                Ok(hash) => hash,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            return match Users::change_password(&pool,&username,&password_hashed).await {
                Ok(_) => { HttpResponse::NoContent().finish()}
                Err(_) => { HttpResponse::InternalServerError().finish()}
            }
        }
        _ => {return HttpResponse::InternalServerError().finish()}
    }
}