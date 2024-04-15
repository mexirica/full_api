use sqlx::SqlitePool;

use crate::models::fornecedor::Fornecedor;
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Fornecedor> for Fornecedor {
    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Fornecedor>, sqlx::Error> {
        let row = sqlx::query_as!(Fornecedor, r#"SELECT * FROM fornecedor WHERE id = $1"#, id)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &SqlitePool) -> Result<Vec<Fornecedor>, sqlx::Error> {
        let rows = sqlx::query_as!(Fornecedor, "SELECT * FROM fornecedor")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &SqlitePool, item: Fornecedor) -> Result<(), sqlx::Error> {
        let fornecedor: Fornecedor = item.into();
        let rows_affected = sqlx::query!(
            r#"INSERT INTO fornecedor (id,nome,documento,tipo_fornecedor,ativo)
        VALUES ($1,$2,$3,$4,$5)"#,
            fornecedor.id,
            fornecedor.nome,
            fornecedor.documento,
            fornecedor.tipo_fornecedor,
            fornecedor.ativo
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query!(r#"DELETE FROM fornecedor WHERE id = $1"#, id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
