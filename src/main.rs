//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .

use mongodb::{self, Client};
use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("unable to connect to database");

    let database = client.database("test");
    println!("database name: {}", database.name());

    let health_route = warp::path!("health").and_then(health_handler);

    let retrieve_activity_route =
        warp::path!("retrieve_activity").and_then(retrieve_activity_handler);
    let upload_activity_route = warp::path!("upload_activity").and_then(upload_activity_handler);

    let routes = health_route
        .or(retrieve_activity_route)
        .or(upload_activity_route)
        .with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}

async fn retrieve_activity_handler() -> Result<impl Reply> {
    //let test_collection = client.database("test").collection("testCollection");
    /*let test_ok = test_collection
    .find_one()
    .await?
    .expect("Missing 'testitem' document.");*/
    Ok("testitem successfully retrieved")
}

async fn upload_activity_handler() -> Result<impl Reply> {
    Ok("OK")
}
