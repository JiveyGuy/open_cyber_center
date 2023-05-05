use std::{env, vec};
use bson::oid::ObjectId;
use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use mongodb::{options::{ClientOptions, ResolverConfig}};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct GameStruct {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    year: String,
    rating: String,
    video_url: String,
    img_url: String,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    print!("debug info:");
    // println!("{}", cursor.);

    let vec_cursor = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    println!("len: {}", vec_cursor.len());


    // for entry in vec_cursor {
    //     println!("{}", entry.name);
    // }
    
    let mut buffer_string = String::new();
    buffer_string.push('[');
    
    for (i, entry) in vec_cursor.iter().enumerate() {
        buffer_string.push_str(&serde_json::to_string(&entry).unwrap());
        if i < vec_cursor.len() - 1 {
            buffer_string.push_str(", ");
        }
    }
    
    buffer_string.push(']');
    println!("{}", buffer_string);

    let mut file = File::create("output.json")?;
    file.write_all(buffer_string.as_bytes())?;
    Ok(())


}