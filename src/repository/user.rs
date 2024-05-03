use sqlx::{Error, SqlitePool};

use crate::models::users::{UserRepository, Users};
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Users> for UserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Users>, Error> {
        let row = sqlx::query_as!(Users, r#"SELECT * FROM Users WHERE username = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Users>, Error> {
        let rows = sqlx::query_as!(Users, "SELECT * FROM Users")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn save(&self, item: Users) -> Result<(), Error> {
        let user: Users = item.into();
        sqlx::query!(
            r#"INSERT INTO users (username, password, refresh_token) VALUES ($1, $2, $3)"#,
            user.username,
            user.password,
            user.refresh_token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, item: Users) -> Result<(), Error> {
        let user: Users = item.into();
        let _rows_affected = sqlx::query!(
            r#"UPDATE Users SET password = $1, refresh_token = $2 WHERE username = $3"#,
            user.password,
            user.refresh_token,
            user.username
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        let _rows_affected = sqlx::query!(r#"DELETE FROM Users WHERE username = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}