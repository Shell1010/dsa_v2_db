use crate::models::Report;
use csv::{ReaderBuilder, StringRecord};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;


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

        let column_names = rows.into_iter().map(|x| x.get::<String, _>(1)).collect();
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

    pub async fn import_csv_to_db(&self, csv_path: &str) {
        let contents = match Self::read_csv_file(csv_path).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read CSV file: {e}");
                return;
            }
        };

        let column_names = match self.get_column_names().await {
            Ok(cols) => cols,
            Err(e) => {
                eprintln!("Failed to get column names: {e}");
                return;
            }
        };

        let query_template = Self::get_query_template(&column_names);

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_slice());
        
        let headers = match rdr.headers().cloned() {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Failed to read CSV headers: {e}");
                return;
            }
        };

        let mut tx = match self.pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to begin transaction: {e}");
                return;
            }
        };

        for result in rdr.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error reading CSV record: {e}");
                    continue;
                }
            };

            let map = Self::process_csv_record(&headers, &record);

            let mut query = sqlx::query(&query_template);
            for col in &column_names {
                query = match map.get(col.as_str()).cloned().flatten() {
                    Some(val) => query.bind(val),
                    None => query.bind(None::<String>),
                };
            }

            if let Err(e) = query.execute(&mut *tx).await {
                eprintln!("Failed to insert row: {e}");
                continue;
            }
        }

        if let Err(e) = tx.commit().await {
            eprintln!("Failed to commit transaction: {e}");
        }
    }

    async fn read_csv_file(path: &str) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        Ok(contents)
    }

    fn get_query_template(columns: &[String]) -> String {
        let columns_str = columns.join(", ");
        let placeholders = std::iter::repeat_n("?", columns.len())
            .collect::<Vec<_>>()
            .join(", ");
        format!("INSERT INTO reports ({columns_str}) VALUES ({placeholders})")
    }


    fn process_csv_record<'a>(
        headers: &'a StringRecord,
        record: &'a StringRecord,
    ) -> HashMap<&'a str, Option<String>> {
        let mut map: HashMap<&str, Option<String>> = headers
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k, Some(v.to_string())))
            .collect();

        if let Some(uid) = map.get("platform_uid").cloned().flatten() {
            let parts: Vec<&str> = uid.split('-').collect();
            if parts.len() >= 3 {
                map.insert("report_id", Some(parts[0].to_string()));
                map.insert("target_id", Some(parts[1].to_string()));
                map.insert("report_type", Some(parts[2].to_string()));
            } else {
                map.insert("report_id", None);
                map.insert("target_id", None);
                map.insert("report_type", None);
            }
        }

        map
    }
}
