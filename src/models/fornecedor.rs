use serde::{Deserialize, Serialize};
use crate::models::DTO;

pub enum TipoFornecedor{
    PessoaFisica = 1,
    PessoaJuridica = 2
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Fornecedor {
    pub id: i64,
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: i64,
    pub ativo: bool
}



impl Fornecedor{
    pub fn new(nome: String,documento: String,tipo_fornecedor: i64) -> Self {
        Self{ id: 0, nome,documento,tipo_fornecedor,ativo:true}
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct NewFornecedor {
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: i64,
}
impl DTO for NewFornecedor{}
impl From<NewFornecedor> for Fornecedor {
    fn from(fornecedor: NewFornecedor) -> Self {
        Fornecedor::new(fornecedor.nome,fornecedor.documento,fornecedor.tipo_fornecedor)
    }
}