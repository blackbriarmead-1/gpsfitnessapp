use crate::{get_mongo_connection, Activity, OIDS};
use mongodb::{self, bson::doc};

pub async fn activity_get(oids: OIDS) -> Result<impl warp::Reply, warp::Rejection> {
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
        Err(warp::reject())
    } else {
        Ok(warp::reply::json(&found_activities))
    }
}
