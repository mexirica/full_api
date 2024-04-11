use uuid::Uuid;
use crate::models::endereco::Endereco;

pub enum TipoFornecedor{
    PessoaFisica = 1,
    PessoaJuridica = 2
}

pub struct Fornecedor {
    pub id: Uuid,
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: TipoFornecedor,
    pub endereco: Endereco,
    pub ativo: bool
}

impl Fornecedor{
    pub fn new(nome: String,documento: String,tipo_fornecedor: TipoFornecedor,endereco: Endereco) -> Self {
        Self{id: Uuid::new_v4(),nome,documento,tipo_fornecedor,endereco,ativo:true}
    }
}