use std::{env};
use bson::oid::ObjectId;
use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use mongodb::{options::{ClientOptions, ResolverConfig}};

#[derive(Serialize, Deserialize, Debug)]
struct GameStruct {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    year: String,
    video_url: String,
    img_url: String,
}



#[tokio::main]
async fn main() {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("error");
    let client = mongodb::Client::with_options(options).unwrap();

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await.expect("error") {
        println!("- {}", name);
    }

    // Get the 'movies' collection from the 'sample_mflix' database:
    let db = client.database("test");
    
    println!("Collections:");
    for name in db.list_collection_names(None).await.expect("error") {
        println!("- {}", name);
    }

    let games = db.collection::<GameStruct>("GameList");


    println!("Indexs:");
    for name in games.list_index_names().await.expect("error") {
        println!("- {}", name);
    }

    let cursor = games.find(None, None).await.expect("error");

    let vec_cursor = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    println!("len: {}", vec_cursor.len());
}