use crate::{get_mongo_connection, Activity, OIDS};

use mongodb::{self, bson::doc};

pub async fn activity_delete(oids: OIDS) -> Result<impl warp::Reply, warp::Rejection> {
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
