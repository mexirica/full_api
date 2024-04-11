CREATE TABLE Endereco (
                          fornecedor_id INTEGER PRIMARY KEY AUTOINCREMENT ,
                          logradouro TEXT NOT NULL,
                          numero TEXT,
                          complemento TEXT,
                          cep TEXT NOT NULL,
                          bairro TEXT NOT NULL,
                          cidade TEXT NOT NULL,
                          estado TEXT NOT NULL
);
