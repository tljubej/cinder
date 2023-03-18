use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
