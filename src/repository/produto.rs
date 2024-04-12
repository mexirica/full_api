use std::fmt::Debug;

use sqlx::{Acquire, Error, SqlitePool};
use sqlx::prelude::*;
use crate::models::DTO;
use crate::models::produto::{Produto, NewProduto};
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Produto> for Produto {

    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Produto>, Error> {
        let mut conn = pool.acquire().await?;
        let row = sqlx::query_as!(Produto, r#"SELECT * FROM produto WHERE id = $1"#, id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &SqlitePool) -> Result<Vec<Produto>, Error> {
        let mut conn = pool.acquire().await?;
        let rows = sqlx::query_as!(Produto, "SELECT * FROM produto")
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &SqlitePool, item: Produto) -> Result<(), Error> {
        let produto: Produto = item.into();
        let mut conn = pool.acquire().await?;
        let _rows_affected = sqlx::query!(
            r#"INSERT INTO Produto ( nome, imagem, valor, data_cadastro, fornecedores_id, ativo)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            produto.nome, produto.imagem, produto.valor, produto.data_cadastro, produto.fornecedores_id, produto.ativo)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), Error> {
        let mut conn = pool.acquire().await?;
        let _rows_affected = sqlx::query!(
            r#"DELETE FROM Produto WHERE id = $1"#,
            id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}
