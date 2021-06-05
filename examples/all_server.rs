use futures::join;
use hexagonapp_rest::{RestServer, RestServerState};
use hexagonapp_grpc::{GrpcServer, GrpcServerState};
use std::net::SocketAddr;
use hexagonapp_postgres::{PostgresStudentRepository};
use hexagonapp_mysql::MySqlStudentRepository;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "sqlx=info");

    println!("Attempting to connect to the postgres pool...");

    println!("Connected to postgres pool !");

    println!("--- Rest Server initialization ---");

    let mysql_database_uri = "mysql://root:password@localhost/test";
    let mysql_pool = hexagonapp_mysql::connect(mysql_database_uri).await;

    // Note: If there's multiple repositories, we need to clone the pool (it's an Arc pointer so it's okay)
    let student_repository = MySqlStudentRepository::new(mysql_pool);
    let rest_server_state = RestServerState::new(student_repository);

    let rest_server = RestServer::new(rest_server_state);
    println!("--- Rest Server initialized ---");

    let postgres_database_uri = "postgres://postgres:password@localhost/test";
    let postgres_pool = hexagonapp_postgres::connect(postgres_database_uri).await;

    // Note: If there's multiple repositories, we need to clone the pool (it's an Arc pointer so it's okay)
    let student_repository = PostgresStudentRepository::new(postgres_pool);

    let grpc_address: SocketAddr = "[::1]:50051".parse().unwrap();
    let grpc_server_state = GrpcServerState::new(student_repository);

    let grpc_server = GrpcServer::new(grpc_server_state);

    let address: [u8; 4] = [127, 0, 0, 1];
    let rest_port: u16 = 3030;

    join!(rest_server.run(address, rest_port), grpc_server.run(grpc_address));
}