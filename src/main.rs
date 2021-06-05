use crate::application::rest::RestServer;
use crate::application::grpc::GrpcServer;
use futures::join;

mod application;

#[tokio::main]
async fn main() {
    println!("Attempting to connect to the postgres pool...");

    println!("Connected to postgres pool !");

    println!("--- Rest Server initialization ---");

    let rest_server = RestServer::new().await;
    println!("--- Rest Server initialized ---");

    let grpc_server = GrpcServer::new();

    let address: [u8; 4] = [127, 0, 0, 1];
    let rest_port: u16 = 3030;

    join!(rest_server.run(address, rest_port), grpc_server.run());
}