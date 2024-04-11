use uuid::Uuid;

pub struct Endereco {
    pub fornecedor_id: Uuid,
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
        fornecedor_id: Uuid,logradouro: String, numero: String,
       complemento: String,cep:String,bairro:String,cidade:String, estado:String)
        -> Self {
        Self{fornecedor_id,logradouro,numero,complemento,cep,bairro,cidade,estado}
    }
}