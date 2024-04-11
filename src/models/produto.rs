use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Serialize,Deserialize,FromRow)]
pub struct Produto {
    pub id: Uuid,
    pub nome: String,
    pub imagem: String,
    pub valor: f32,
    pub data_cadastro: DateTime<Utc>,
    pub fornecedores_id: String,
    pub ativo: bool
}

impl Produto {
    pub fn new(
        nome: String,
        imagem: String,
        valor: f32,
        fornecedores_id: String,
    ) -> Self {
        Self {
            id : Uuid::new_v4(),
            nome,
            imagem,
            valor,
            data_cadastro : Utc::now(),
            ativo : true,
            fornecedores_id
        }
    }
}