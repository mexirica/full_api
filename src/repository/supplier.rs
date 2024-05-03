use sqlx::SqlitePool;

use crate::models::supplier::{Supplier, SupplierRepository};
use crate::repository::Repository;

#[async_trait::async_trait]
impl Repository<Supplier> for SupplierRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Supplier>, sqlx::Error> {
        let row = sqlx::query_as!(Supplier, r#"SELECT * FROM supplier WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Supplier>, sqlx::Error> {
        let rows = sqlx::query_as!(Supplier, "SELECT * FROM supplier")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn save(&self,item: Supplier) -> Result<(), sqlx::Error> {
        let supplier: Supplier = item.into();
        let result = sqlx::query!(
            r#"INSERT INTO supplier (id,name,supplier_type,active)
        VALUES ($1,$2,$3,$4)"#,
            supplier.id,
            supplier.name,
            supplier.supplier_type,
            supplier.active
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
        
    }

    async fn update(&self, item: Supplier) -> Result<(), sqlx::Error> {
        let supplier: Supplier = item.into();
        let result = sqlx::query!(
            r#"UPDATE supplier SET name = $1, supplier_type = $3, active = $4 WHERE id = $5"#,
            supplier.name,
            supplier.supplier_type,
            supplier.active,
            supplier.id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }    }

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(r#"DELETE FROM supplier WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }    }
}
