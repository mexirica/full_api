
CREATE TABLE Supplier (
                          id INTEGER PRIMARY KEY AUTOINCREMENT ,
                          name TEXT NOT NULL,
                          supplier_type INTEGER NOT NULL,
                          active BOOLEAN NOT NULL,
                          costumer_username TEXT NOT NULL
);
