use sqlx::{Error, SqlitePool};

mod produto;
mod fornecedor;
mod endereco;

pub trait Repository<T>{
    fn find_by_id(pool: &SqlitePool, id: i32) -> Result<Option<T>, Error>;
    fn find_all(pool: &SqlitePool) -> Result<Vec<T>, Error>;
    fn save(pool: &SqlitePool, item: T) -> Result<(), Error>;
    fn delete(pool: &SqlitePool, id: i32) -> Result<(), Error>;
}

pub async fn find_all<T: Repository<T>>(pool: &SqlitePool) -> Result<Vec<T>, Error>{
    T::find_all(pool).await
}

pub async fn find_by_id<T: Repository<T>>(pool: &SqlitePool, id: i32) -> Result<Option<T>, Error> {
    T::find_by_id(pool,id).await
}

pub async fn save<T: Repository<T>>(pool: &SqlitePool, model: T) -> Result<(),Error> {
T::save(pool,model).await
}

pub async fn delete<T: Repository<T>>(pool: &SqlitePool, id: i32) -> Result<(),Error> {
    T::delete(pool,id).await
}