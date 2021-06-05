mod student_entity;
mod student_repository;

use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;

pub async fn connect(uri: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(uri).await
        .expect("Can't open a pool with postgres")
}

pub use student_repository::PostgresStudentRepository;
