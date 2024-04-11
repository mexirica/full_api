use sqlx::PgPool;

use sqlx::Error;
use crate::repository::Repository;
use crate::models::fornecedor::Fornecedor;
#[async_trait::async_trait]
impl Repository<Fornecedor> for Fornecedor{

    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Fornecedor>, sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let row = sqlx::query_as!(Fornecedor,r#"SELECT * FROM fornecedor WHERE id = $1"#,id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &PgPool) -> Result<Vec<Fornecedor>, sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows = sqlx::query_as!(models::Fornecedor,"SELECT * FROM fornecedor")
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &PgPool, item: Fornecedor) -> Result<(), sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"INSERT INTO fornecedor (title,created_at,body,category)
        VALUES ($1,NOW(),$2,$3)"#,item.title,item.body,item.category)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        let pool = pool.lock().unwrap();
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"DELETE FROM fornecedor WHERE id = $1"#,id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}

