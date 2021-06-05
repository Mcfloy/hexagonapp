use tonic::transport::Server;
use sqlx::postgres::PgPoolOptions;
use crate::application::grpc::student::proto_student::grpc_student_service_server::{GrpcStudentServiceServer};
use crate::application::grpc::student::StudentController;
use std::sync::Arc;
use postgres_repository::SqlStudentRepository;

pub struct GrpcServer {}

impl GrpcServer {
    pub fn new() -> Self {
        GrpcServer {}
    }

    pub async fn run(&self) {
        let address = "[::1]:50051".parse().unwrap();

        let database = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test").await
            .expect("Can't open a pool with postgres");

        let student_repository = SqlStudentRepository::new(database);
        let student_controller = StudentController::new(Arc::new(student_repository));

        println!("Grpc server listening to {:?}", address);

        Server::builder()
            .add_service(GrpcStudentServiceServer::new(student_controller))
            .serve(address)
            .await
            .unwrap();
    }
}