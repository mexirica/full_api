use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, SqlitePool};

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
pub struct UserRepository {
    pub pool: SqlitePool
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool: (pool) }
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn delete_by_username(&self, username: String) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM users WHERE username = $1"#, username)
            .execute(&self.pool)
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
        .execute(&self.pool)
        .await?;

        Ok(())
}
}