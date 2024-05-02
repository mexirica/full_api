use sqlx::{Error, SqlitePool};

use crate::models::produto::{Produto, ProdutoRepository};
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Produto> for ProdutoRepository<'_> {
    async fn find_by_id(&self, id: i64) -> Result<Option<Produto>, Error> {
        let result = sqlx::query_as!(Produto, r#"SELECT * FROM produto WHERE id = $1"#, id)
            .fetch_optional(self.pool.unwrap())
            .await?;

        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<Produto>, Error> {
        let rows = sqlx::query_as!(Produto, "SELECT * FROM produto")
            .fetch_all(self.pool.unwrap())
            .await?;

        Ok(rows)
    }

    async fn save(&self, item: Produto) -> Result<(), Error> {
        let produto: Produto = item.into();
        let _rows_affected = sqlx::query!(
            r#"INSERT INTO Produto ( nome, imagem, valor, data_cadastro, fornecedores_id, ativo)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            produto.nome,
            produto.imagem,
            produto.valor,
            produto.data_cadastro,
            produto.fornecedores_id,
            produto.ativo
        )
        .execute(self.pool.unwrap())
        .await?;

        Ok(())
    }

    async fn update(&self, item: Produto) -> Result<(), Error> {
        let produto: Produto = item.into();
        let _rows_affected = sqlx::query!(
            r#"UPDATE produto SET nome = $1, imagem = $2, valor = $3, data_cadastro = $4, fornecedores_id = $5, ativo = $6 WHERE id = $7"#,
            produto.nome,
            produto.imagem,
            produto.valor,
            produto.data_cadastro,
            produto.fornecedores_id,
            produto.ativo,
            produto.id
        )
        .execute(self.pool.unwrap())
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM produto WHERE id = $1"#, id)
            .execute(self.pool.unwrap())
            .await?;

        Ok(())
    }
}
