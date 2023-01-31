use crate::{Activity};
use crate::util::util::get_mongo_connection;

pub async fn activity_post(activity: Activity) -> Result<impl warp::Reply, warp::Rejection> {
    let database = get_mongo_connection().await;
    let test_collection = database.collection("testCollection");
    let result = test_collection.insert_one(activity, None).await;
    match result {
        Ok(_) => Ok("Successfully uploaded activity"),
        Err(_) => Err(warp::reject()),
    }
}
