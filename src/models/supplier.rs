use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub enum SupplierType {
    Physics  = 1,
    Juridic  = 2,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Supplier {
    pub id: i64,
    pub name: String,
    pub supplier_type: i64,
    pub active: bool,
    pub costumer_username: String
}

impl Supplier {
    pub fn new(name: String, supplier_type: i64,costumer_username: String) -> Self {
        Self {
            id: 0,
            name,
            supplier_type,
            active: true,
            costumer_username
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewSupplier {
    pub name: String,
    pub supplier_type: i64,
}
#[derive(Clone)]
pub struct SupplierRepository {
    pub pool: SqlitePool
}

impl<'a> SupplierRepository {
        pub fn new(pool: SqlitePool) -> Self {
            Self { pool: (pool) }
        }

}