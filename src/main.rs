//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .
//https://github.com/Mr-Malomz/rocket-mongo-api/tree/main/src
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
//https://stackoverflow.com/questions/53887738/server-selection-timeout-error-mongodb-go-driver-with-docker

mod models;

use chrono::{Utc, TimeZone};
use models::activity_model::Activity;
use models::GPSTS_model::GPSTS;
use models::HRTS_model::HRTS;
use mongodb::{
    self, bson::{doc, DateTime}, options::ClientOptions, results::InsertOneResult, Client, Database,
};
use serde;
use warp::{http, reject, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route =
        warp::get()
        .and(warp::path!("health"))
        .and_then(health_handler);

    let retrieve_activity_route =
        warp::get()
        .and(warp::path!("retrieve_activity"))
        .and_then(retrieve_activity_handler);
    let upload_activity_route = 
        warp::post()
        .and(warp::path!("upload_activity"))
        .and_then(upload_activity_handler);

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

pub async fn get_mongo_connection() -> Database {
    let client = Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .unwrap();

    client.database("test")
}

async fn retrieve_activity_handler() -> Result<impl Reply> {
    Ok("OK")
}

async fn upload_activity_handler() -> Result<impl Reply> {
    println!("running upload activity handler");
    let database = get_mongo_connection().await;
    println!("got database connection");
    let test_collection = database.collection("testCollection");
    println!("got test_collection");

    /*let mut gpsvec = vec![];
    let gpsitem = GPSTS{
        timestamp: None,
        lat: 0.0,
        lon: 0.0,
    };
    gpsvec.push(gpsitem);*/

    let new_doc = Activity {
        id: None,
        length: 10.0,
        activity_title: "outdoor run".to_string(),
        //gps: gpsvec,
        gps: None,
        hr: None,
    };
    println!("new doc initialized");

    let result = test_collection.insert_one(new_doc, None).await;
    println!("result done awaiting");
    match result {
        Ok(_) => Ok("Successfully uploaded activity"),
        Err(_) => Err(reject()),
    }
}
