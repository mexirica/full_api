use std::env;
use dotenv::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use crate::models::product::ProductRepository;
use crate::models::supplier::SupplierRepository;
use crate::models::users::UserRepository;
pub (crate) mod repository;

pub async fn connect() -> SqlitePool {
    dotenv().ok();
    let base_path = env::current_dir().expect("Failed to determine the current directory");
    let database_url = base_path.join("api.db");
    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await.expect("Failed to connect to DB")
}

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