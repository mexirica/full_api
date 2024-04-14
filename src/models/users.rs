use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize,FromRow)]

pub struct Users {
    pub username: String,
    pub password: String,
    pub refresh_token: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser{
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePassword{
    pub password: String
}

#[derive(Debug,Deserialize)]
pub struct UsernameQuery{
    pub username: String
}
impl Users {
    pub async fn find_by_username(pool: &SqlitePool, username: &String) -> Result<Option<Users>, Error> {
        let row = sqlx::query_as!(Users, r#"SELECT * FROM users WHERE username = $1"#, username)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    pub async fn delete_by_username(pool: &SqlitePool, username: String) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM users WHERE username = $1"#, username)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn change_password(pool: &SqlitePool, username: &String, password: &String) -> Result<(), Error> {
        sqlx::query!(r#"UPDATE users SET password = $1 WHERE username = $2"#, password,username)
            .execute(pool)
            .await?;

        Ok(())
    }
}