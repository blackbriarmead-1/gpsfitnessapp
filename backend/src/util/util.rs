use mongodb::{Client,Database};



/// json_body is a macro that allows for json to be used as input to handler functions
/// 
/// Warp uses filters to handle api calls, json_body takes a Struct as input
/// to use as a template for JSON. Serde will verify that the JSON is valid
/// as determined by the struct which includes serde traits and options
/// 
/// This macro prevents having to make a function to make a filter for each
/// Struct we want to interpret from JSON
#[macro_export]
macro_rules! json_body {
    ($input:expr) => {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    };
}

/// get_mongo_connection is a function to connect to the database.
/// 
/// Putting this here allows every handler to have the same logic for connecting to the database
/// so if the ip changes, for example, it will only need to change here
/// 
/// Future functionality may include the ability to select a specific database,
/// or a separate function just for getting the client and not the database
pub async fn get_mongo_connection() -> Database {
    let client = Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .unwrap();

    client.database("test")
}