use sqlx::{Error, SqlitePool};
use crate::models::DTO;

mod produto;
mod fornecedor;
mod endereco;
#[async_trait::async_trait]
pub trait Repository<T>{
    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<T>, Error>;
    async fn find_all(pool: &SqlitePool) -> Result<Vec<T>, Error>;
    async fn save(pool: &SqlitePool, item: dyn DTO) -> Result<(), Error>;
    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), Error>;
}

pub async fn find_all<T: Repository<T>>(pool: &SqlitePool) -> Result<Vec<T>, Error>{
    T::find_all(pool).await
}

pub async fn find_by_id<T: Repository<T>>(pool: &SqlitePool, id: i64) -> Result<Option<T>, Error> {
    T::find_by_id(pool,id).await
}

pub async fn save<T: Repository<T>>(pool: &SqlitePool, model: E) -> Result<(),Error> {
T::save(pool,model).await
}

pub async fn delete<T: Repository<T>>(pool: &SqlitePool, id: i64) -> Result<(),Error> {
    T::delete(pool,id).await
}