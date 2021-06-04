use sqlx::{Pool, Postgres};
use crate::infrastructure::repository::postgres::student_entity::StudentEntity;
use async_trait::async_trait;
use library::student::repository::StudentRepository;
use library::student::Student;
use uuid::Uuid;
use library::student::error::StudentError;

pub struct SqlStudentRepository {
    pool: Pool<Postgres>
}

impl SqlStudentRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        SqlStudentRepository {
            pool
        }
    }
}

#[async_trait]
impl StudentRepository for SqlStudentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, StudentError> {
        match sqlx::query_as::<_, StudentEntity>("SELECT * FROM students WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await {
            Ok(student) => Ok(Some(student.to_domain_entity())),
            Err(error) => Err(StudentError::new(error.to_string()))
        }
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), StudentError> {
        if let Err(error) = sqlx::query("DELETE * FROM students WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await {
            return Err(StudentError::new(error.to_string()));
        }
        Ok(())
    }

    async fn create(&self, student: Student) -> Result<Uuid, StudentError> {
        if let Err(error) = sqlx::query("INSERT INTO students (id, firstname, lastname, gpa) VALUES ($1, $2, $3, $4)")
            .bind(student.get_id())
            .bind(student.get_firstname())
            .bind(student.get_lastname())
            .bind(student.get_gpa())
            .execute(&self.pool)
            .await {
            return Err(StudentError::new(error.to_string()));
        }
        Ok(student.get_id().clone())
    }
}