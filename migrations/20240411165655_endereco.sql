CREATE TABLE Endereco (
                          fornecedor_id TEXT PRIMARY KEY,
                          logradouro TEXT NOT NULL,
                          numero TEXT,
                          complemento TEXT,
                          cep TEXT NOT NULL,
                          bairro TEXT NOT NULL,
                          cidade TEXT NOT NULL,
                          estado TEXT NOT NULL
);
