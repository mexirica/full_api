
CREATE TABLE Users (
    username TEXT NOT NULL PRIMARY KEY UNIQUE ,
    password TEXT NOT NULL,
    refresh_token TEXT NOT NULL
)