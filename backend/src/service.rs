use bson::doc;
use futures::stream::StreamExt;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    nickname: String,
    company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyComplaint {
    company_name: String,
    complaint: String,
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

pub fn add_company_complaint_route(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("add_company")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(add_company_complaint)
}

pub async fn add_company_complaint(
    db: Database,
    company: CompanyComplaint,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Adding company {:?}", company);

    db.collection::<CompanyComplaint>("company_issues")
        .insert_one(company, None)
        .await
        .map(|_| warp::reply::json(&"Success"))
        .map_err(|err| warp::reject::custom(Error(err.to_string())))
}

pub fn get_company_complaints_route(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_company_issues" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_company_complaints)
}

pub async fn get_company_complaints(
    company_name: String,
    db: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Getting complaints for company {}", company_name);

    let res = db
        .collection::<CompanyComplaint>("company_complaints")
        .find(doc! {"company_name": company_name}, None)
        .await
        .map_err(|err| warp::reject::custom(Error(err.to_string())))?;

    let res = res
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| Error(err.to_string()))?;

    Ok(warp::reply::json(&res))
}

pub fn get_employee_count_route(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_company_issues" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_employee_count)
}

pub async fn get_employee_count(
    company_name: String,
    db: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Getting employee count for company {}", company_name);

    let res = db
        .collection::<Worker>("workers")
        .find(doc! {"company_name": company_name}, None)
        .await
        .map_err(|err| warp::reject::custom(Error(err.to_string())))?;

    let res = res
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| Error(err.to_string()))?;

    Ok(warp::reply::json(&res.len()))
}
