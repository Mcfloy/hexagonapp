use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    id: Uuid,
    firstname: String,
    lastname: String,
    gpa: f32
}

impl Student {
    pub fn new(id: Uuid, firstname: String, lastname: String, gpa: f32) -> Self {
        Student {
            id,
            firstname,
            lastname,
            gpa
        }
    }

    pub fn get_id(&self) -> Uuid { self.id }

    pub fn get_firstname(&self) -> String { String::from(&self.firstname) }

    pub fn get_lastname(&self) -> String { String::from(&self.lastname) }

    pub fn get_gpa(&self) -> f32 { self.gpa }

    pub fn set_gpa(&mut self, new_gpa: f32) {
        self.gpa = new_gpa;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let student_id = Uuid::new_v4();
        let student = Student::new(student_id, "Tony".to_string(), "Stark".to_string(), 4.0);
        assert_eq!(student_id, student.id);
        assert_eq!("Tony", student.firstname);
        assert_eq!("Stark", student.lastname);
        assert_eq!(4.0, student.gpa);
    }
}