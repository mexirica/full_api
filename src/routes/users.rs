use actix_web::{HttpResponse, post, Responder, web};
use sqlx::SqlitePool;
use crate::models::users::*;
use crate::auth;
use crate::auth::{compute_password_hash, JwtResponse};
#[post("")]
pub async fn create_user(pool: web::Data<SqlitePool>, user: web::Json<NewUser>) -> impl Responder {
    let user = user.into_inner();
    match sqlx::query_as!(User,r#"SELECT * FROM users WHERE username = $1"#, user.username).
        fetch_optional(pool.get_ref())
        .await{
        Ok(None) => {
                let mut access_token = String::new();
                let mut refresh_token = String::new();
            if let Ok(hash) = compute_password_hash(user.password) {
               let password_hashed = hash;
               if let Ok(access) = auth::create_access_token(&user.username).await{
                    access_token = access;
               }else{
                   return HttpResponse::InternalServerError().finish()
               };
                if let Ok(refresh) = auth::create_refresh_token(&user.username).await{
                     refresh_token = refresh;
                }else{
                    return HttpResponse::InternalServerError().finish();
                }



                let result = sqlx::query!(r#"INSERT INTO users (username,password,refresh_token)
                VALUES ($1,$2,$3)"#,&user.username,&user.password,&refresh_token)
                    .execute(pool.get_ref())
                    .await;

                match result {
                    Ok(_) => {HttpResponse::Created().json(JwtResponse{access_token,refresh_token})}
                    Err(_) => {HttpResponse::InternalServerError().finish()}
                }
            }else{

                HttpResponse::InternalServerError().finish()
            }
        }
        Ok(Some(_)) => {return HttpResponse::BadRequest().finish()}
        _ => {return HttpResponse::BadRequest().finish()}
    }
}