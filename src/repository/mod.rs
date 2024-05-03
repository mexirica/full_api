use sqlx::Error;

mod supplier;
mod product;
mod user;
pub (crate) mod uow;

#[async_trait::async_trait]
pub trait Repository<T> {
    async fn find_by_id(&self, id: i64) -> Result<Option<T>, Error>;
    async fn find_all(&self) -> Result<Vec<T>, Error>;
    async fn save(&self, item: T) -> Result<(), Error>;
    async fn update (&self, item: T) -> Result<(), Error>;
    async fn delete(&self, id: i64) -> Result<(), Error>;
}


