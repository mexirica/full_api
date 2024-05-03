use std::rc::Rc;
use std::sync::Arc;
use sqlx::{Pool, Sqlite, SqlitePool};
use crate::models::fornecedor::FornecedorRepository;
use crate::models::produto::ProdutoRepository;
use crate::models::users::UserRepository;
#[derive(Clone)]
pub struct UnitOfWork {
    pub fornecedor: FornecedorRepository,
    pub user: UserRepository,
    pub produto: ProdutoRepository,
}

impl UnitOfWork {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            fornecedor: FornecedorRepository::new(pool.clone()),
            user: UserRepository::new(pool.clone()),
            produto: ProdutoRepository::new(pool.clone()),
        }

    }
}