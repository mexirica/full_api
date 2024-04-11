use sqlx::Error;
use sqlx::SqlitePool;
use crate::repository::Repository;
use crate::models::produto::Produto;
#[async_trait::async_trait]
impl Repository<Produto> for Produto{

    async fn find_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Produto>, sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let row = sqlx::query_as!(Produto,r#"SELECT * FROM produto WHERE id = $1"#,id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &SqlitePool) -> Result<Vec<Produto>, sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows = sqlx::query_as!(models::Produto,"SELECT * FROM produto")
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &SqlitePool, item: Produto) -> Result<(), sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"INSERT INTO Produto (title,created_at,body,category)
        VALUES ($1,NOW(),$2,$3)"#,item.title,item.body,item.category)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn delete(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"DELETE FROM Produto WHERE id = $1"#,id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}

