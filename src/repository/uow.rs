use sqlx::SqlitePool;

use crate::models::product::ProductRepository;
use crate::models::supplier::SupplierRepository;
use crate::models::users::UserRepository;

#[derive(Clone)]
pub struct UnitOfWork {
    pub supplier: SupplierRepository,
    pub user: UserRepository,
    pub product: ProductRepository,
}

impl UnitOfWork {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            supplier: SupplierRepository::new(pool.clone()),
            user: UserRepository::new(pool.clone()),
            product: ProductRepository::new(pool),
        }

    }
}