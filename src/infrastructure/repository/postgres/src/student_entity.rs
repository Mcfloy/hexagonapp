use uuid::Uuid;
use library::student::Student;

#[derive(sqlx::FromRow)]
pub struct StudentEntity {
    id: Uuid,
    firstname: String,
    lastname: String,
    gpa: f32
}

impl StudentEntity {
    pub fn to_student(&self) -> Student {
        Student::new(self.id, String::from(&self.firstname), String::from(&self.lastname), self.gpa)
    }
}