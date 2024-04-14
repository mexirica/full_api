
use sqlx::{Error, SqlitePool};

use crate::models::users::Users;
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Users> for Users {

    async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Users>, Error> {
        let row = sqlx::query_as!(Users, r#"SELECT * FROM Users WHERE username = $1"#, id)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    async fn find_all(pool: &SqlitePool) -> Result<Vec<Users>, Error> {
        let rows = sqlx::query_as!(Users, "SELECT * FROM Users")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    async fn save(pool: &SqlitePool, item: Users) -> Result<(), Error> {
        let user: Users = item.into();
        sqlx::query!(
        r#"INSERT INTO users (username, password, refresh_token) VALUES ($1, $2, $3)"#,
        user.username,
        user.password,
        user.refresh_token
    )
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), Error> {
        let _rows_affected = sqlx::query!(
            r#"DELETE FROM Users WHERE username = $1"#,
            id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
