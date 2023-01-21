//https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
//docker build -t rust-debian -f Dockerfile .
//https://github.com/Mr-Malomz/rocket-mongo-api/tree/main/src
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
//https://stackoverflow.com/questions/53887738/server-selection-timeout-error-mongodb-go-driver-with-docker
//https://mudge.name/oplog/doc/mongodb/coll/options/index.html

mod models;

use chrono::{TimeZone, Utc};
use futures_util::stream::StreamExt;
use models::gpsts_model::GPSTS;
use models::hrts_model::HRTS;
use models::{activity_me_options::ActivityMeOptions, activity_model::Activity, oid_model::OIDS};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use mongodb::{
    self,
    bson::{doc, DateTime},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Database,
};
use serde;
use warp::hyper::StatusCode;
use warp::{http, reject, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route = warp::get()
        .and(warp::path!("health"))
        .and_then(health_handler);

    let retrieve_activity_route = warp::get()
        .and(warp::path!("activity"))
        .and(json_body_oids())
        .and_then(retrieve_activity_handler);

    let retrieve_activity_me_route = warp::get()
        .and(warp::path!("activity" / "me"))
        .and(json_body_activity_me())
        .and_then(retrieve_activity_me_handler);

    let upload_activity_route = warp::post()
        .and(warp::path!("activity"))
        .and(json_body_activity())
        .and_then(upload_activity_handler);

    let delete_activity_route = warp::delete()
        .and(warp::path!("activity"))
        .and(json_body_oids())
        .and_then(delete_activity_handler);

    let routes = health_route
        .or(retrieve_activity_route)
        .or(retrieve_activity_me_route)
        .or(upload_activity_route)
        .or(delete_activity_route)
        .with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn json_body_activity() -> impl Filter<Extract = (Activity,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_oids() -> impl Filter<Extract = (OIDS,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_activity_me(
) -> impl Filter<Extract = (ActivityMeOptions,), Error = warp::Rejection> + Clone {
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

async fn retrieve_activity_handler(oids: OIDS) -> Result<impl Reply> {
    let database = get_mongo_connection().await;
    let test_collection: mongodb::Collection<Activity> = database.collection("testCollection");
    let mut found_activities: Vec<Activity> = vec![];

    //let filter = doc! { "_id": oids.oids.first() };

    for oid in oids.oids.iter() {
        let filter = doc! { "_id": oid };
        //let find_options = mongodb::options::FindOptions::builder().projection(doc! { "book": 1, "_id": 0 }).build();
        let result = test_collection.find_one(filter, None).await;

        match result {
            Ok(_) => {
                let response = result.expect("does not contain activity");
                match response {
                    Some(_) => found_activities.push(response.unwrap()),
                    None => (),
                }
            }
            Err(_) => (),
        }
    }

    if found_activities.len() == 0 {
        Err(reject())
    } else {
        Ok(warp::reply::json(&found_activities))
    }
}

async fn retrieve_activity_me_handler(options: ActivityMeOptions) -> Result<impl Reply> {
    let database = get_mongo_connection().await;
    let test_collection: mongodb::Collection<Activity> = database.collection("testCollection");
    let mut found_activities: Vec<Activity> = vec![];
    //THIS PART MUST CHANGE ONCE USERS ARE IMPLEMENTED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    let find_options = mongodb::options::FindOptions::builder()
        .batch_size(options.limit)
        .limit(options.limit as i64)
        .skip(options.offset as u64)
        .allow_partial_results(false)
        .sort(doc! {"end_time": -1})
        .build();

    let result = test_collection.find(None, find_options).await;

    match result {
        Ok(_) => {
            let mut cursor = result.expect("does not contain activity");
            while let Some(doc) = cursor.next().await {
                match doc {
                    Ok(_) => found_activities.push(doc.unwrap()),
                    Err(_) => (),
                }
            }
            if found_activities.len() == 0 {
                Err(warp::reject())
            } else {
                Ok(warp::reply::json(&found_activities))
            }
        }
        Err(_) => Err(warp::reject()),
    }
}

async fn delete_activity_handler(oids: OIDS) -> Result<impl Reply> {
    let database = get_mongo_connection().await;
    let test_collection: mongodb::Collection<Activity> = database.collection("testCollection");
    let mut deleted_activities = 0;

    //let filter = doc! { "_id": oids.oids.first() };

    for oid in oids.oids.iter() {
        let filter = doc! { "_id": oid };
        //let find_options = mongodb::options::FindOptions::builder().projection(doc! { "book": 1, "_id": 0 }).build();
        let result = test_collection.delete_one(filter, None).await;

        match result {
            Ok(_) => {
                //deleted may return Ok() without actually deleting anything
                //check if something was deleted before incrementing deleted_activities
                deleted_activities += result.unwrap().deleted_count;
            }
            Err(_) => (),
        }
    }
    Ok(warp::reply::json(
        &doc! {"num_deleted_activities": deleted_activities as i64},
    ))
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
