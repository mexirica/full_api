CREATE TABLE Produto (
                         id TEXT PRIMARY KEY,
                         nome TEXT NOT NULL,
                         imagem TEXT,
                         valor REAL NOT NULL,
                         data_cadastro DATETIME NOT NULL,
                         fornecedores_id TEXT NOT NULL,
                         ativo BOOLEAN NOT NULL
);
