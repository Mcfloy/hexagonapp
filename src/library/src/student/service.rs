use crate::student::Student;
use crate::student::repository::StudentRepository;
use uuid::Uuid;
use std::sync::Arc;
use crate::student::error::StudentError;

pub async fn get<Repo: StudentRepository>(id: Uuid, repository: Arc<Repo>) -> Result<Option<Student>, StudentError> {
    repository.find_by_id(id).await
}

pub async fn create<Repo: StudentRepository>(student: Student, repository: Arc<Repo>) -> Result<Uuid, StudentError> {
    repository.create(student).await
}

pub async fn delete<Repo: StudentRepository>(id: Uuid, repository: Arc<Repo>) -> Result<(), StudentError> {
    repository.delete_by_id(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;
    use crate::student::Student;
    use uuid::Uuid;
    use crate::student::repository::StudentRepository;
    use crate::student::error::StudentError;
    use std::borrow::Borrow;
    use async_trait::async_trait;

    pub struct InMemoryStudentRepository {
        database: Mutex<HashMap<String, Student>>
    }

    impl InMemoryStudentRepository {
        pub fn new() -> Self {
            let map = HashMap::<String, Student>::new();
            InMemoryStudentRepository {
                database: Mutex::new(map)
            }
        }
    }

    #[async_trait]
    impl StudentRepository for InMemoryStudentRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, StudentError> {
            let uuid = id.to_hyphenated().to_string();
            match self.database.lock().unwrap().get(&uuid) {
                None => Ok(None),
                Some(student) => Ok(Some(student.clone()))
            }
        }

        async fn delete_by_id(&self, id: Uuid) -> Result<(), StudentError> {
            let uuid = id.to_hyphenated().to_string();
            match self.database.lock().unwrap().remove(&uuid) {
                None => Err(StudentError::new(format!("Student {} Not Found", uuid).to_string())),
                Some(_) => Ok(())
            }
        }

        async fn create(&self, student: Student) -> Result<Uuid, StudentError> {
            let uuid = student.get_id().clone();
            match self.database.lock().unwrap().insert(uuid.to_hyphenated().to_string(), student) {
                None => Ok(uuid),
                Some(_) => Ok(uuid)
            }
        }
    }

    #[tokio::test]
    async fn test_create() {
        let student_repo = Arc::new(InMemoryStudentRepository::new());
        let uuid = Uuid::new_v4();
        let student = Student::new(uuid, "Tony".to_string(), "Stark".to_string(), 4.0);
        let result = create(student, student_repo).await;
        assert_eq!(true, result.is_ok());
        assert_eq!(uuid, result.unwrap());
    }
}