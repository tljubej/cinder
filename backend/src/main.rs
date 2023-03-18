mod service;

use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let client_options = mongodb::options::ClientOptions::parse(
        "mongodb://sinderDbUser:sinderPassword@localhost:27017/sinder",
    )
    .await
    .expect("Cannot parse clientoptions");

    let client = mongodb::Client::with_options(client_options).expect("Cannot create mongo client");

    let db = client.database("sinder");

    let hello = service::signup_route(db.clone())
        .or(service::add_company_complaint_route(db.clone()))
        .or(service::get_company_complaints_route(db.clone()))
        .or(service::get_meeting_info_route(db.clone()))
        .or(service::add_meeting_route(db.clone()))
        .or(service::get_employee_count_route(db.clone()));

    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
