use sqlx::SqlitePool;

use crate::models::fornecedor::{Fornecedor, FornecedorRepository};
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Fornecedor> for FornecedorRepository<'_> {
    async fn find_by_id(&self, id: i64) -> Result<Option<Fornecedor>, sqlx::Error> {
        let row = sqlx::query_as!(Fornecedor, r#"SELECT * FROM fornecedor WHERE id = $1"#, id)
            .fetch_optional(self.pool.unwrap())
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Fornecedor>, sqlx::Error> {
        let rows = sqlx::query_as!(Fornecedor, "SELECT * FROM fornecedor")
            .fetch_all(self.pool.unwrap())
            .await?;

        Ok(rows)
    }

    async fn save(&self,item: Fornecedor) -> Result<(), sqlx::Error> {
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
        .execute(self.pool.unwrap())
        .await?;

        Ok(())
    }

    async fn update(&self, item: Fornecedor) -> Result<(), sqlx::Error> {
        let fornecedor: Fornecedor = item.into();
        let rows_affected = sqlx::query!(
            r#"UPDATE fornecedor SET nome = $1, documento = $2, tipo_fornecedor = $3, ativo = $4 WHERE id = $5"#,
            fornecedor.nome,
            fornecedor.documento,
            fornecedor.tipo_fornecedor,
            fornecedor.ativo,
            fornecedor.id
        )
        .execute(self.pool.unwrap())
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query!(r#"DELETE FROM fornecedor WHERE id = $1"#, id)
            .execute(self.pool.unwrap())
            .await?;

        Ok(())
    }
}
