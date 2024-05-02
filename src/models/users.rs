use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Sqlite, SqlitePool};
use crate::models::produto::ProdutoRepository;

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct Users {
    pub username: String,
    pub password: String,
    pub refresh_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePassword {
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UsernameQuery {
    pub username: String,
}
impl Users {
}
#[derive(Clone)]
pub struct UserRepository<'a> {
    pub pool: Option<&'a Pool<Sqlite>>
}
    impl Default for UserRepository<'_> {
        fn default() -> Self {
            Self { pool: None }
        }
    }

impl UserRepository<'_> {
    pub fn new(pool: & Pool<Sqlite>) -> Self {
        Self { pool: Some(pool) }
    }

    pub async fn find_by_username(
        &self,
        username: &String,
    ) -> Result<Option<Users>, Error> {
        let row = sqlx::query_as!(
            Users,
            r#"SELECT * FROM users WHERE username = $1"#,
            username
        )
        .fetch_optional(self.pool.unwrap())
        .await?;

        Ok(row)
    }

    pub async fn delete_by_username(&self, username: String) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM users WHERE username = $1"#, username)
            .execute(self.pool.unwrap())
            .await?;

        Ok(())
    }

    pub async fn change_password(
        &self,
        username: &String,
        password: &String,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"UPDATE users SET password = $1 WHERE username = $2"#,
            password,
            username
        )
        .execute(self.pool.unwrap())
        .await?;

        Ok(())
}
}