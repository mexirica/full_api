CREATE TABLE Product (
                         id INTEGER PRIMARY KEY AUTOINCREMENT ,
                         name TEXT NOT NULL,
                         image TEXT,
                         price REAL NOT NULL,
                         created_at DATETIME NOT NULL,
                         supplier_id INTEGER NOT NULL,
                         active BOOLEAN NOT NULL
);