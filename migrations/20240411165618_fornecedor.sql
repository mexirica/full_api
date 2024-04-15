CREATE TABLE Fornecedor (
                            id INTEGER PRIMARY KEY AUTOINCREMENT ,
                            nome TEXT NOT NULL,
                            documento TEXT NOT NULL,
                            tipo_fornecedor INTEGER NOT NULL,
                            ativo BOOLEAN NOT NULL,
                            cliente_username TEXT NOT NULL
);
