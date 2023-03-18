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

    let hello = service::signup_route(db.clone()).or(service::add_company_route(db.clone()));

    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
