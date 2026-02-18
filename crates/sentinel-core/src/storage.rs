// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use sqlx::{Pool, Postgres, Sqlite};
use anyhow::{Result, Context};
use crate::config::AgentConfig;
use crate::state::AIOpsRcaEvent;

#[derive(Debug, Clone)]
pub enum StorageEngine {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
    Memory,
}

#[derive(Debug, Clone)]
pub struct Storage {
    engine: StorageEngine,
}

impl Storage {
    pub async fn new(config: &AgentConfig) -> Result<Self> {
        // 1. Try PostgreSQL / TimescaleDB if URL provided
        if let Some(url) = &config.database.database_url {
            tracing::info!("Storage: Connecting to PostgreSQL/Timescale at configured URL...");
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(config.database.max_connections)
                .connect(url).await
                .context("Failed to connect to PostgreSQL")?;
            
            // Auto-migrate schema
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS rca_events (
                    id UUID PRIMARY KEY,
                    gpu_id TEXT NOT NULL,
                    timestamp_ms BIGINT NOT NULL,
                    root_cause TEXT NOT NULL,
                    confidence DOUBLE PRECISION NOT NULL, 
                    details TEXT NOT NULL
                );
                CREATE TABLE IF NOT EXISTS risk_assessments (
                    gpu_id TEXT NOT NULL,
                    timestamp_ms BIGINT NOT NULL,
                    failure_probability DOUBLE PRECISION NOT NULL,
                    risk_score DOUBLE PRECISION NOT NULL,
                    factors JSONB NOT NULL
                );
                "#
            ).execute(&pool).await?;

            return Ok(Self { engine: StorageEngine::Postgres(pool) });
        }

        // 2. Default: SQLite (Embedded)
        if config.database.enable_local_db {
            let path = format!("sqlite://{}/sentinel.db?mode=rwc", config.local_tsdb_path);
            tracing::info!("Storage: Initializing embedded SQLite DB at {}", path);
            
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&path).await
                .context("Failed to open SQLite DB")?;

            // Auto-migrate
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS rca_events (
                    id TEXT PRIMARY KEY,
                    gpu_id TEXT NOT NULL,
                    timestamp_ms INTEGER NOT NULL,
                    root_cause TEXT NOT NULL,
                    confidence REAL NOT NULL, 
                    details TEXT NOT NULL
                );
                CREATE TABLE IF NOT EXISTS risk_assessments (
                    gpu_id TEXT NOT NULL,
                    timestamp_ms INTEGER NOT NULL,
                    failure_probability REAL NOT NULL,
                    risk_score REAL NOT NULL,
                    factors TEXT NOT NULL
                );
                "#
            ).execute(&pool).await?;

            return Ok(Self { engine: StorageEngine::Sqlite(pool) });
        }

        Ok(Self { engine: StorageEngine::Memory })
    }

    pub async fn save_rca_event(&self, event: &AIOpsRcaEvent) -> Result<()> {
        let event_id = uuid::Uuid::new_v4();
        match &self.engine {
            StorageEngine::Postgres(pool) => {
                sqlx::query(
                    "INSERT INTO rca_events (id, gpu_id, timestamp_ms, root_cause, confidence, details) VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(event_id)
                .bind(&event.gpu_id)
                .bind(event.timestamp_ms as i64)
                .bind(&event.root_cause)
                .bind(event.confidence)
                .bind(&event.details)
                .execute(pool).await?;
            },
            StorageEngine::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO rca_events (id, gpu_id, timestamp_ms, root_cause, confidence, details) VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(event_id.to_string())
                .bind(&event.gpu_id)
                .bind(event.timestamp_ms as i64)
                .bind(&event.root_cause)
                .bind(event.confidence)
                .bind(&event.details)
                .execute(pool).await?;
            },
            StorageEngine::Memory => {}
        }
        Ok(())
    }

    pub async fn get_recent_rca_events(&self, limit: i64) -> Result<Vec<AIOpsRcaEvent>> {
        let events = match &self.engine {
            StorageEngine::Postgres(pool) => {
                sqlx::query_as::<_, PgRcaEvent>(
                    "SELECT * FROM rca_events ORDER BY timestamp_ms DESC LIMIT $1"
                )
                .bind(limit)
                .fetch_all(pool).await?
                .into_iter().map(|e| AIOpsRcaEvent {
                    gpu_id: e.gpu_id,
                    timestamp_ms: e.timestamp_ms as u64,
                    root_cause: e.root_cause,
                    confidence: e.confidence,
                    details: e.details,
                }).collect()
            },
            StorageEngine::Sqlite(pool) => {
                sqlx::query_as::<_, SqliteRcaEvent>(
                    "SELECT * FROM rca_events ORDER BY timestamp_ms DESC LIMIT ?"
                )
                .bind(limit)
                .fetch_all(pool).await?
                .into_iter().map(|e| AIOpsRcaEvent {
                    gpu_id: e.gpu_id,
                    timestamp_ms: e.timestamp_ms as u64,
                    root_cause: e.root_cause,
                    confidence: e.confidence,
                    details: e.details,
                }).collect()
            },
            StorageEngine::Memory => vec![]
        };
        Ok(events)
    }
}

// Internal structs for SQL mapping
#[derive(sqlx::FromRow)]
struct PgRcaEvent {
    #[allow(dead_code)]
    id: uuid::Uuid,
    gpu_id: String,
    timestamp_ms: i64,
    root_cause: String,
    confidence: f64,
    details: String,
}

#[derive(sqlx::FromRow)]
struct SqliteRcaEvent {
    #[allow(dead_code)]
    id: String,
    gpu_id: String,
    timestamp_ms: i64,
    root_cause: String,
    confidence: f64,
    details: String,
}
