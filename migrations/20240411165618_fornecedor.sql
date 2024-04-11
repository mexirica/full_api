CREATE TABLE Fornecedor (
                            id TEXT PRIMARY KEY,
                            nome TEXT NOT NULL,
                            documento TEXT NOT NULL,
                            tipo_fornecedor INTEGER NOT NULL,
                            endereco_id TEXT NOT NULL,
                            ativo BOOLEAN NOT NULL
);
