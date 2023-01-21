//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .
//https://github.com/Mr-Malomz/rocket-mongo-api/tree/main/src
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
//https://stackoverflow.com/questions/53887738/server-selection-timeout-error-mongodb-go-driver-with-docker

mod models;

use chrono::{TimeZone, Utc};
use models::activity_model::Activity;
use models::gpsts_model::GPSTS;
use models::hrts_model::HRTS;
use mongodb::{
    self,
    bson::{doc, DateTime},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Database,
};
use serde;
use warp::{http, reject, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route = warp::get()
        .and(warp::path!("health"))
        .and_then(health_handler);

    let retrieve_activity_route = warp::get()
        .and(warp::path!("activity"))
        .and_then(retrieve_activity_handler);

    let upload_activity_route = warp::post()
        .and(warp::path!("activity"))
        .and(json_body())
        .and_then(upload_activity_handler);

    let routes = health_route
        .or(retrieve_activity_route)
        .or(upload_activity_route)
        .with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn json_body() -> impl Filter<Extract = (Activity,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}

pub async fn get_mongo_connection() -> Database {
    let client = Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .unwrap();

    client.database("test")
}

async fn retrieve_activity_handler() -> Result<impl Reply> {
    let database = get_mongo_connection().await;
    let test_collection: mongodb::Collection<Activity> = database.collection("testCollection");
    let result = test_collection.find_one(None, None).await;

    match result {
        Ok(_) => {
            let response = result.expect("does not contain activity").unwrap();
            Ok(warp::reply::json(&response))
        }
        Err(_) => Err(reject()),
    }
}

async fn upload_activity_handler(activity: Activity) -> Result<impl Reply> {
    let database = get_mongo_connection().await;
    let test_collection = database.collection("testCollection");
    let result = test_collection.insert_one(activity, None).await;
    match result {
        Ok(_) => Ok("Successfully uploaded activity"),
        Err(_) => Err(reject()),
    }
}
