// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, vec};
use bson::oid::ObjectId;
use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use mongodb::{options::{ClientOptions, ResolverConfig}};
// use std::fs::File;

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
    exe_url: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn update_entry(_id: i16, _name: &str, _description: &str, _year: &str, _rating: &str, _video_url: &str, _img_url: &str, _exe_url: &str) -> Result<String, ()> {
    // println!("I was called with game name: \"{}\"", name);
    // println!("I was called with game description: \"{}\"", description);
    // println!("I was called with game year: \"{}\"", year);
    // println!("I was called with game rating: \"{}\"", rating);
    // println!("I was called with game video_url: \"{}\"", video_url);
    // println!("I was called with game img_url: \"{}\"", img_url);
    // println!("I was called with game exe_url: \"{}\"", exe_url);

    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("error");
    let client = mongodb::Client::with_options(options).unwrap();

    let db = client.database("test");

    let games = db.collection::<GameStruct>("GameList");

    let cursor = games.find(None, None).await.expect("error");

    let vec_cursor = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

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

    // let mut _file = File::create("output.json")?;
    // file.write_all(buffer_string.as_bytes())?;


    Ok("".to_string())
}


#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
