use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

pub enum TipoFornecedor {
    PessoaFisica = 1,
    PessoaJuridica = 2,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Fornecedor {
    pub id: i64,
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: i64,
    pub ativo: bool,
    pub cliente_username: String
}

impl Fornecedor {
    pub fn new(nome: String, documento: String, tipo_fornecedor: i64,cliente_username: String) -> Self {
        Self {
            id: 0,
            nome,
            documento,
            tipo_fornecedor,
            ativo: true,
            cliente_username
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewFornecedor {
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: i64,
}
#[derive(Clone)]
pub struct FornecedorRepository {
    pub pool: SqlitePool
}

impl<'a> FornecedorRepository {
        pub fn new(pool: SqlitePool) -> Self {
            Self { pool: (pool) }
        }

}