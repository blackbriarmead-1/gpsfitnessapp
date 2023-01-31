use crate::{Activity, ActivityMeOptions};
use crate::util::util::get_mongo_connection;
use futures_util::stream::StreamExt;
use mongodb::{self, bson::doc};

pub async fn activity_me_get(
    options: ActivityMeOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
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
