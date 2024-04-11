use crate::models::DTO;
use crate::models::produto::NewProduto;

pub enum TipoFornecedor{
    PessoaFisica = 1,
    PessoaJuridica = 2
}

pub struct Fornecedor {
    pub id: i64,
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: TipoFornecedor,
    pub endereco_id: String,
    pub ativo: bool
}



impl Fornecedor{
    pub fn new(nome: String,documento: String,tipo_fornecedor: TipoFornecedor,endereco_id: String) -> Self {
        Self{ id: 0, nome,documento,tipo_fornecedor,endereco_id,ativo:true}
    }
}

pub struct NewFornecedor {
    pub nome: String,
    pub documento: String,
    pub tipo_fornecedor: TipoFornecedor,
    pub endereco: String,
}
impl DTO for NewFornecedor{}
impl From<NewFornecedor> for Fornecedor {
    fn from(fornecedor: NewFornecedor) -> Self {
        Fornecedor::new(fornecedor.nome,fornecedor.documento,fornecedor.tipo_fornecedor,fornecedor.endereco)
    }
}