use tonic::{Request, Response, Status};
use tonic::transport::Server;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloResponse, HelloRequest};
use sqlx::postgres::PgPoolOptions;
use crate::infrastructure::repository::postgres::SqlStudentRepository;
use crate::application::grpc::student::proto_student::grpc_student_service_server::{GrpcStudentServiceServer, GrpcStudentService};
use crate::application::grpc::student::StudentController;
use std::sync::Arc;

pub mod hello_world {
    tonic::include_proto!("grpc_helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

pub struct GrpcServer {}

impl GrpcServer {
    pub fn new() -> Self {
        GrpcServer {}
    }

    pub async fn run(&self) {
        let address = "[::1]:50051".parse().unwrap();
        let greeter = MyGreeter::default();

        let database = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test").await
            .expect("Can't open a pool with postgres");

        let student_repository = SqlStudentRepository::new(database);
        let student_controller = StudentController::new(Arc::new(student_repository));

        println!("Grpc server listening to {:?}", address);

        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .add_service(GrpcStudentServiceServer::new(student_controller))
            .serve(address)
            .await
            .unwrap();
    }
}