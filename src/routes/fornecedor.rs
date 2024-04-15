use std::future::Future;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use sqlx::SqlitePool;

use crate::auth::jwt;
use crate::models::fornecedor::{Fornecedor, NewFornecedor};
use crate::repository::Repository;

pub mod configure {
    use actix_web::web;

    use crate::routes::fornecedor::*;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/fornecedores")
            .service(get_fornecedor)
        );
    }
}

#[get("/{fornecedor_id}")]
pub async fn get_fornecedor(pool: web::Data<SqlitePool>, id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();

    return Fornecedor::find_by_id(&pool, id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .map_or(HttpResponse::NotFound().finish(), |fornecedor| HttpResponse::Ok().json(fornecedor))
}

#[post("")]
pub async fn create_fornecedor(pool: web::Data<SqlitePool>,fornecedor: web::Json<NewFornecedor>,req: HttpRequest) -> impl Responder {
    let fornecedor = fornecedor.into_inner();
    match jwt::get_claims(&req) {
        Ok(claim) => {
            let fornecedor = Fornecedor{nome: fornecedor.nome, documento: fornecedor.documento, tipo_fornecedor: fornecedor.tipo_fornecedor,ativo: true, cliente_username: claim.sub,id: 0};
           return match Fornecedor::save(&pool,fornecedor).await {
                Ok(_) => {HttpResponse::Created().finish()}
                Err(_) => {HttpResponse::InternalServerError().finish()}
            }

        }
        Err(e) => {
            return HttpResponse::InternalServerError().finish()
        }
    }
}