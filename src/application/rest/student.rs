mod filters {
    use std::sync::Arc;
    use warp::Filter;
    use super::handlers;
    use library::student::repository::StudentRepository;

    pub fn routes<Repo>(student_repository: Arc<Repo>)
        -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
        where Repo: StudentRepository + Send + Sync {
        get_student(student_repository.clone())
    }

    pub fn get_student<Repo>(student_repository: Arc<Repo>)
        -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
        where Repo: StudentRepository + Send + Sync {
        warp::path!("students" / String)
            .and(warp::get())
            .and(with_repo(student_repository))
            .and_then(handlers::get_student)
    }

    fn with_repo<Repo>(student_repository: Arc<Repo>) -> impl Filter<Extract = (Arc<Repo>,), Error = std::convert::Infallible> + Clone
        where Repo: StudentRepository + Send + Sync {
        warp::any().map(move || student_repository.clone())
    }
}

mod handlers {
    use std::convert::Infallible;
    use library::student::repository::StudentRepository;
    use std::sync::Arc;
    use library::student::{service};
    use uuid::Uuid;
    use warp::http::StatusCode;
    use warp::Reply;

    pub async fn get_student<Repo>(id: String, student_repository: Arc<Repo>) -> Result<warp::reply::Response, Infallible>
        where Repo: StudentRepository + Send + Sync {
        let id = Uuid::parse_str(&id);
        if id.is_err() {
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
        match service::get(id.unwrap(), student_repository).await {
            Ok(student) => {
                match student {
                    None => Ok(StatusCode::NOT_FOUND.into_response()),
                    Some(student) => Ok(warp::reply::json(&student).into_response())
                }
            }
            Err(_error) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub use filters::routes;