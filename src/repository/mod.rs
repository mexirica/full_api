use sqlx::{Error, SqlitePool};
use crate::models::DTO;

mod produto;
mod fornecedor;
mod endereco;
#[async_trait::async_trait]
pub trait Repository<T>{
    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<T>, Error>;
    async fn find_all(pool: &SqlitePool) -> Result<Vec<T>, Error>;
    async fn save(pool: &SqlitePool, item: T) -> Result<(), Error>;
    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), Error>;
}


