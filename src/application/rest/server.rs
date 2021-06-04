use std::sync::Arc;
use warp::Filter;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub struct RestServer {
    database: Pool<Postgres>
}

impl RestServer {
    pub async fn new() -> Self {
        let database = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test").await
            .expect("Can't open a pool with postgres");

        // Instantiate StudentRepository with a pool clone
        // Instantiate StudentService with the StudentRepository
        // store the generated routes in a map
        RestServer {
            database
        }
    }

    pub async fn run(&self, address: [u8; 4], port: u16) {
        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {}", name));

        println!("Rest server listening to {:?}:{:?}", address, port);

        warp::serve(hello)
            .run((address, port))
            .await
    }
}