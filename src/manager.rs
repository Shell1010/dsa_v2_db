use std::error::Error;

use crate::models::Report;
use sqlx::{Row, SqlitePool};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> sqlx::Result<Self> {
        let db_url = format!("sqlite://{db_path}");
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_column_names(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query("PRAGMA table_info(reports)")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let column_names = rows.into_iter()
            .map(|x| x.get::<String, _>(1))
            .collect();
        Ok(column_names)
    }

    pub async fn get_all_reports(&self) -> sqlx::Result<Vec<Report>> {
        let reports = sqlx::query_as::<_, Report>("SELECT * FROM reports")
            .fetch_all(&self.pool)
            .await?;
        Ok(reports)
    }

    pub async fn get_reports_by_target_id(&self, target_id: &str) -> sqlx::Result<Vec<Report>> {
        sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE target_id = ? ORDER BY created_at DESC",
        )
        .bind(target_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn execute_raw(&self, query: &str) -> sqlx::Result<()> {
        sqlx::query(query).execute(&self.pool).await?;
        Ok(())
    }
}
