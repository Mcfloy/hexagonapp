use proto_student::{GetStudentRequest, GetStudentResponse, AddStudentRequest, AddStudentResponse};
use proto_student::grpc_student_service_server::GrpcStudentService;
use std::sync::{Arc};
use tonic::{Status, Response, Request};
use uuid::Uuid;
use library::student::repository::StudentRepository;
use library::student::{service, Student};

pub mod proto_student {
    tonic::include_proto!("student");
}

#[derive(Debug)]
pub struct StudentController<R>
    where R : StudentRepository + Send + Sync{
    student_repository: Arc<R>
}

impl <R> StudentController<R>
    where R : StudentRepository + Send + Sync {
    pub fn new(student_repository: Arc<R>) -> Self {
        StudentController {
            student_repository
        }
    }
}

#[tonic::async_trait]
impl <R: 'static> GrpcStudentService for StudentController<R>
    where R: StudentRepository + Send + Sync {
    async fn get_student(&self, request: Request<GetStudentRequest>) -> Result<Response<GetStudentResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id).unwrap();
        match service::get(id, self.student_repository.clone()).await {
            Ok(student) => {
                match student {
                    None => {
                        Err(Status::not_found("Student was not found"))
                    }
                    Some(student) => {
                        let response = GetStudentResponse {
                            id: student.get_id().to_hyphenated().to_string(),
                            firstname: student.get_firstname(),
                            lastname: student.get_lastname(),
                            gpa: student.get_gpa()
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(error) => Err(Status::internal(error.to_string()))
        }
    }

    async fn add_student(&self, request: Request<AddStudentRequest>) -> Result<Response<AddStudentResponse>, Status> {
        let id = Uuid::new_v4();
        let request = request.into_inner();
        let student = Student::new(id, request.firstname, request.lastname, request.gpa);
        match service::create(student, self.student_repository.clone()).await {
            Ok(id) => {
                let response = AddStudentResponse { id: id.to_hyphenated().to_string() };
                Ok(Response::new(response))
            }
            Err(error) => Err(Status::internal(error.to_string()))
        }
    }
}