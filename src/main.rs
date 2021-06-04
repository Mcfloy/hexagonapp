use crate::application::rest::RestServer;
use crate::infrastructure::repository::postgres::SqlStudentRepository;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use crate::application::grpc::GrpcServer;
use futures::join;

mod application;
mod infrastructure;

#[tokio::main]
async fn main() {
    println!("Attempting to connect to the postgres pool...");

    println!("Connected to postgres pool !");

    println!("--- Rest Server initialization ---");

    let rest_server = RestServer::new().await;
    println!("--- Rest Server initialized ---");

    let grpc_server = GrpcServer::new();

    let address = [127, 0, 0, 1];
    let rest_port = 3030;


    join!(rest_server.run(address, rest_port), grpc_server.run());
}