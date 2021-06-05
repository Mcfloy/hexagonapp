use warp::Filter;
use std::sync::Arc;
use hexagonapp::student::repository::StudentRepository;

pub struct RestServer<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    state: RestServerState<StudentRepo>
}

pub struct RestServerState<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    student_repository: Arc<StudentRepo>
}

impl <StudentRepo> RestServerState<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    pub fn new(student_repository: StudentRepo) -> Self {
        RestServerState {
            student_repository: Arc::new(student_repository)
        }
    }
}

impl <StudentRepo: 'static> RestServer<StudentRepo>
    where StudentRepo: StudentRepository + Send + Sync {
    pub fn new(server_state: RestServerState<StudentRepo>) -> Self {
        RestServer {
            state: server_state
        }
    }

    pub async fn run(&self, address: [u8; 4], port: u16) {
        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {}", name));

        println!("Rest server listening to {:?}:{:?}", address, port);

        let routes = super::student::routes(self.state.student_repository.clone())
            .or(hello);

        warp::serve(routes)
            .run((address, port))
            .await
    }
}