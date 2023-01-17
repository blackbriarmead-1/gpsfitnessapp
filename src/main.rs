//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .
//https://github.com/Mr-Malomz/rocket-mongo-api/tree/main/src
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
//https://stackoverflow.com/questions/53887738/server-selection-timeout-error-mongodb-go-driver-with-docker

mod models;

use models::activity_model::Activity;
use mongodb::{
    self, bson::doc, options::ClientOptions, results::InsertOneResult, Client, Database,
};
use serde;
use warp::{http, reject, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
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

pub async fn get_mongo_connection() -> Database {
    let client = Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .unwrap();

    client.database("test")
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
    println!("running upload activity handler");
    let database = get_mongo_connection().await;
    println!("got database connection");
    let test_collection = database.collection("testCollection");
    println!("got test_collection");

    let new_doc = Activity {
        id: None,
        length: 10.0,
        activity_title: "outdoor run".to_string(),
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
