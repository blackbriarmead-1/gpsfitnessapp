//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .
//https://github.com/Mr-Malomz/rocket-mongo-api/tree/main/src
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
//https://stackoverflow.com/questions/53887738/server-selection-timeout-error-mongodb-go-driver-with-docker
//https://mudge.name/oplog/doc/mongodb/coll/options/index.html
//https://www.scottbrady91.com/openssl/creating-rsa-keys-using-openssl
//https://github.com/rustls/hyper-rustls/issues/115

mod api;
mod handlers;
mod models;
mod routes;
use models::{activity_me_options::ActivityMeOptions, activity_model::Activity, oid_model::OIDS};
use mongodb::{self, Client, Database};
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, Rejection>;

use self::{
    handlers::{
        activity_delete_handler, activity_get_handler, activity_me_get_handler,
        activity_post_handler, health_get_handler,
    },
    routes::{
        activity_delete_route, activity_get_route, activity_me_get_route, activity_post_route,
        health_get_route,
    },
};

#[macro_export]
macro_rules! json_body {
    ($input:expr) => {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    };
}

#[tokio::main]
async fn main() {
    let routes = health_get!()
        .or(activity_get!())
        .or(activity_me_get!())
        .or(activity_post!())
        .or(activity_delete!())
        .with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

pub async fn get_mongo_connection() -> Database {
    let client = Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .unwrap();

    client.database("test")
}
