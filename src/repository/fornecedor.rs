use sqlx::SqlitePool;
use crate::models::DTO;

use crate::models::fornecedor::{Fornecedor, NewFornecedor};
use crate::repository::Repository;

pub type TNewFornecedor = NewFornecedor;
#[async_trait::async_trait]
impl Repository<Fornecedor> for Fornecedor{

    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Fornecedor>, sqlx::Error> {
        let mut conn = pool.acquire().await?;
        let row = sqlx::query_as!(Fornecedor,r#"SELECT * FROM fornecedor WHERE id = $1"#,id)
            .fetch_optional(&mut conn)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &SqlitePool) -> Result<Vec<Fornecedor>, sqlx::Error> {
        let mut conn = pool.acquire().await?;
        let rows = sqlx::query_as!(Fornecedor,"SELECT * FROM fornecedor")
            .fetch_all(&mut conn)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &SqlitePool, item: NewFornecedor) -> Result<(), sqlx::Error> {
        let fornecedor: Fornecedor = item.into();
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"INSERT INTO fornecedor (id,nome,documento,tipo_fornecedor,endereco_id,ativo)
        VALUES ($1,$2,$3,$4,$5,$6)"#,fornecedor.id,fornecedor.nome,fornecedor.documento,fornecedor.tipo_fornecedor,fornecedor.endereco_id,fornecedor.ativo)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        let mut conn = pool.acquire().await?;
        let rows_affected = sqlx::query!(r#"DELETE FROM fornecedor WHERE id = $1"#,id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

