use sqlx::{Pool, MySql};
use async_trait::async_trait;
use uuid::Uuid;
use crate::student_entity::StudentEntity;
use hexagonapp::student::repository::StudentRepository;
use hexagonapp::student::Student;
use hexagonapp::student::error::StudentError;

pub struct MySqlStudentRepository {
    pool: Pool<MySql>
}

impl MySqlStudentRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        MySqlStudentRepository {
            pool
        }
    }
}

#[async_trait]
impl StudentRepository for MySqlStudentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, StudentError> {
        let id = id.to_hyphenated().to_string().replace("-", "");
        match sqlx::query_as::<_, StudentEntity>("SELECT * FROM students WHERE id = UNHEX(?)")
            .bind(&id)
            .fetch_one(&self.pool)
            .await {
            Ok(student) => Ok(Some(student.to_student())),
            Err(error) => Err(StudentError::new(error.to_string()))
        }
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), StudentError> {
        let id = id.to_hyphenated().to_string().replace("-", "");
        if let Err(error) = sqlx::query("DELETE * FROM students WHERE id = UNHEX(?)")
            .bind(&id)
            .execute(&self.pool)
            .await {
            return Err(StudentError::new(error.to_string()));
        }
        Ok(())
    }

    async fn create(&self, student: Student) -> Result<Uuid, StudentError> {
        let id = student.get_id().to_hyphenated().to_string().replace("-", "");
        if let Err(error) = sqlx::query("INSERT INTO students (id, firstname, lastname, gpa) VALUES (UNHEX(?), ?, ?, ?)")
            .bind(id)
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