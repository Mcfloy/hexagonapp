mod student_entity;
mod student_repository;

use sqlx::{MySql, Pool};
use sqlx::mysql::{MySqlPoolOptions};

pub async fn connect(uri: &str) -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(uri).await
        .expect("Can't open a pool with postgres")
}

pub use student_repository::MySqlStudentRepository;
