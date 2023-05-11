// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn update_entry(id: i16, name: &str, description: &str, year: &str, rating: &str, video_url: &str, img_url: &str, exe_url: &str) -> Result<String, ()> 
{
    // connect to mongo

    // get a cursor to db 
    
    // search mongo db with cursor for game ID
    // find with pattern in mongo for matching id
    
    // when found, update all info 
    //  Make sure it only matches 1 game 

    // Change info to name: &str, description: &str, year: &str, rating: &str, video_url: &str, img_url: &str, exe_url: &str
    // push to mongo

    // make sure to sleep like 1 sec
    write_games_json("./games.json").await.expect("Failed.");
    return Ok(format!("Ok"));
}

async fn write_games_json(path_to_save:&str) -> std::io::Result<()> 
{
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("error");
    let client = mongodb::Client::with_options(options).unwrap();

    // Get the 'movies' collection from the 'sample_mflix' database:
    let db = client.database("test");

    let games = db.collection::<GameStruct>("GameList");
    let cursor = games.find(None, None).await.expect("error");
    let vec_cursor = cursor.try_collect().await.unwrap_or_else(|_| vec![]);  
    
    // ======== Make output string text
    let mut buffer_string = String::new();
    buffer_string.push_str("[\n");
    for (i, entry) in vec_cursor.iter().enumerate() {
        buffer_string.push_str("\n");
        buffer_string.push_str(&serde_json::to_string(&entry).unwrap().replace(",\"", ",\n\t\""));
        
        if i < vec_cursor.len() - 1 {
            buffer_string.push_str(", ");
        }
    
        buffer_string.push_str("\n");
        
    }
    buffer_string.push_str("]\n");
    // =================================

    let mut file = File::create(path_to_save)?;
    file.write_all(buffer_string.as_bytes())?;
    
    Ok(())
}

#[tokio::main]
async fn main() 
{
    write_games_json("./games.json").await.expect("Failed.");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
