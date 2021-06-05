use warp::Filter;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use postgres_repository::SqlStudentRepository;

pub struct RestServer {
    student_repository: Arc<SqlStudentRepository>
}

impl RestServer {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test").await
            .expect("Can't open a pool with postgres");

        // Instantiate StudentRepository with a pool clone
        let student_repository = SqlStudentRepository::new(pool);

        // store the generated routes in a map
        RestServer {
            student_repository: Arc::new(student_repository)
        }
    }

    pub async fn run(&self, address: [u8; 4], port: u16) {
        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {}", name));

        println!("Rest server listening to {:?}:{:?}", address, port);

        let routes = super::student::routes(self.student_repository.clone())
            .or(hello);

        warp::serve(routes)
            .run((address, port))
            .await
    }
}