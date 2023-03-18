use mongodb::Database;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    nickname: String,
    company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    name: String,
    n_employees: u64,
}

#[derive(Debug)]
pub struct Error(String);

impl warp::reject::Reject for Error {}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn signup_route(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("signup")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(signup)
}

pub async fn signup(db: Database, worker: Worker) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Signing up worker {:?}", worker);

    db.collection::<Worker>("workers")
        .insert_one(worker, None)
        .await
        .map(|_| warp::reply::json(&"Success"))
        .map_err(|err| warp::reject::custom(Error(err.to_string())))
}

pub fn add_company_route(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("add_company")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(add_company)
}

pub async fn add_company(
    db: Database,
    company: Company,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Adding company {:?}", company);

    db.collection::<Company>("companies")
        .insert_one(company, None)
        .await
        .map(|_| warp::reply::json(&"Success"))
        .map_err(|err| warp::reject::custom(Error(err.to_string())))
}
