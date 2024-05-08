use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};

use crate::auth::jwt;
use crate::db::repository::Repository;
use crate::db::UnitOfWork;
use crate::models::supplier::{NewSupplier, Supplier};


pub mod configure {
    use actix_web::web;

    use crate::routes::supplier::*;

    pub fn handler(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/suppliers")
            .service(get_supplier)
        );
    }
}

#[get("/{supplier_id}")]
pub async fn get_supplier(uow: web::Data<UnitOfWork>, id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();

    return uow.supplier.find_by_id(id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .map_or(HttpResponse::NotFound().finish(), |supplier| HttpResponse::Ok().json(supplier))
}

#[post("")]
pub async fn create_supplier(uow: web::Data<UnitOfWork>,supplier: web::Json<NewSupplier>,req: HttpRequest) -> impl Responder {
    let supplier = supplier.into_inner();
    match jwt::get_claims(&req) {
        Ok(claim) => {
            let supplier = Supplier{name: supplier.name, supplier_type: supplier.supplier_type,active: true, costumer_username: claim.sub,id: 0};
           return match uow.supplier.save(supplier).await {
                Ok(_) => {HttpResponse::Created().finish()}
                Err(_) => {HttpResponse::InternalServerError().finish()}
            }

        }
        Err(e) => {
            return HttpResponse::InternalServerError().finish()
        }
    }
}