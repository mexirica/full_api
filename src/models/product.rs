use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::models::DTO;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub image: Option<String>,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub supplier_id: i64,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub image: String,
    pub price: f64,
    pub supplier_id: i64,
}
impl Product {
    pub fn new(name: String, image: String, price: f64, supplier_id: i64) -> Self {
        Self {
            id: 0,
            name,
            image: Some(image),
            price,
            created_at: Utc::now().naive_utc(),
            active: true,
            supplier_id,
        }
    }
}

impl From<NewProduct> for Product {
    fn from(new_product: NewProduct) -> Self {
        Product::new(
            new_product.name,
            new_product.image,
            new_product.price,
            new_product.supplier_id,
        )
    }
}

impl DTO for NewProduct {}
#[derive(Clone)]
pub struct ProductRepository {
    pub pool:  SqlitePool
}

impl<'a> ProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool: (pool) }
    }
}