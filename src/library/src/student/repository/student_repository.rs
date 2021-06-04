use uuid::Uuid;
use crate::student::student::Student;
use async_trait::async_trait;
use crate::student::error::StudentError;

#[async_trait]
pub trait StudentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, StudentError>;

    async fn delete_by_id(&self, id: Uuid) -> Result<(), StudentError>;

    async fn create(&self, student: Student) -> Result<Uuid, StudentError>;
}

#[cfg(test)]
mod tests {
    pub struct InMemoryStudentRepository {}
}