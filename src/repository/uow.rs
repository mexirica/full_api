use sqlx::{Pool, Sqlite};

use crate::models::{fornecedor::FornecedorRepository, produto::ProdutoRepository, users::UserRepository};
#[derive(Clone)]
pub struct UnitOfWork<'a> {
    pub pool: Pool<Sqlite>,
    pub fornecedor: FornecedorRepository<'a>,
    pub produto: ProdutoRepository<'a>,
    pub user: UserRepository<'a>,
}

impl<'a> UnitOfWork<'a> {
    pub async fn initialize(&'a mut self) {
        self.fornecedor = FornecedorRepository::new(&self.pool);
        self.produto = ProdutoRepository::new(&self.pool);
        self.user = UserRepository::new(&self.pool);
    }

    pub async fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            fornecedor: FornecedorRepository::default(),
            produto: ProdutoRepository::default(),
            user: UserRepository::default(),
        }
    }
}

