
pub struct Endereco {
    pub fornecedor_id: i64,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String,
    pub cep: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

impl Endereco {
    pub fn new(
        fornecedor_id: i64,logradouro: String, numero: String,
       complemento: String,cep:String,bairro:String,cidade:String, estado:String)
        -> Self {
        Self{fornecedor_id,logradouro,numero,complemento,cep,bairro,cidade,estado}
    }
}