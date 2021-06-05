use crate::student::proto_student::grpc_student_service_server::{GrpcStudentServiceServer};
use crate::student::StudentController;
use std::sync::Arc;
use std::net::SocketAddr;
use hexagonapp::student::repository::StudentRepository;
use tonic::transport::Server;

pub struct GrpcServer<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    state: GrpcServerState<StudentRepo>
}

pub struct GrpcServerState<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    student_repository: Arc<StudentRepo>
}

impl <StudentRepo> GrpcServerState<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    pub fn new(student_repository: StudentRepo) -> Self {
        GrpcServerState {
            student_repository: Arc::new(student_repository)
        }
    }
}

impl <StudentRepo: 'static> GrpcServer<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    pub fn new(server_state: GrpcServerState<StudentRepo>) -> Self {
        GrpcServer {
            state: server_state
        }
    }

    pub async fn run(self, address: SocketAddr) {
        let student_controller = StudentController::new(self.state.student_repository);

        println!("Grpc server listening to {:?}", address);

        Server::builder()
            .add_service(GrpcStudentServiceServer::new(student_controller))
            .serve(address)
            .await
            .unwrap();
    }
}