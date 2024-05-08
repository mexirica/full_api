use sqlx::{Error, SqlitePool};

use crate::models::product::{Product, ProductRepository};
use crate::db::repository::Repository;

#[async_trait::async_trait]
impl Repository<Product> for ProductRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Product>, Error> {
        let result = sqlx::query_as!(Product, r#"SELECT * FROM product WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<Product>, Error> {
        let rows = sqlx::query_as!(Product, "SELECT * FROM product")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn save(&self, item: Product) -> Result<(), Error> {
        let product: Product = item.into();
        let _rows_affected = sqlx::query!(
            r#"INSERT INTO Product ( name, image, price, created_at, supplier_id, active)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            product.name,
            product.image,
            product.price,
            product.created_at,
            product.supplier_id,
            product.active
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, item: Product) -> Result<(), Error> {
        let product: Product = item.into();
        let _rows_affected = sqlx::query!(
            r#"UPDATE product SET name = $1, image = $2, price = $3, created_at = $4, supplier_id = $5, active = $6 WHERE id = $7"#,
            product.name,
            product.image,
            product.price,
            product.created_at,
            product.supplier_id,
            product.active,
            product.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM product WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
