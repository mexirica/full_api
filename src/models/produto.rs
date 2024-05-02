use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

use crate::models::DTO;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Produto {
    pub id: i64,
    pub nome: String,
    pub imagem: Option<String>,
    pub valor: f64,
    pub data_cadastro: NaiveDateTime,
    pub fornecedores_id: String,
    pub ativo: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewProduto {
    pub nome: String,
    pub imagem: String,
    pub valor: f64,
    pub fornecedores_id: String,
}
impl Produto {
    pub fn new(nome: String, imagem: String, valor: f64, fornecedores_id: String) -> Self {
        Self {
            id: 0,
            nome,
            imagem: Some(imagem),
            valor,
            data_cadastro: Utc::now().naive_utc(),
            ativo: true,
            fornecedores_id,
        }
    }
}

impl From<NewProduto> for Produto {
    fn from(new_produto: NewProduto) -> Self {
        Produto::new(
            new_produto.nome,
            new_produto.imagem,
            new_produto.valor,
            new_produto.fornecedores_id,
        )
    }
}

impl DTO for NewProduto {}
#[derive(Clone)]
pub struct ProdutoRepository<'a> {
    pub pool:  Option<&'a Pool<Sqlite>>
}
impl Default for ProdutoRepository<'_> {
    fn default() -> Self {
        Self { pool: None }
    }
}
impl<'a> ProdutoRepository<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        Self { pool: Some(pool) }
    }
}