
use std::{env, vec};
use bson::oid::ObjectId;
// use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use mongodb::{options::{ClientOptions, ResolverConfig}};

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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update_entry(_id: i16, name: &str, description: &str, year: &str, rating: &str, video_url: &str, img_url: &str, exe_url: &str) {
    println!("I was called with game name: \"{}\"", name);
    println!("I was called with game description: \"{}\"", description);
    println!("I was called with game year: \"{}\"", year);
    println!("I was called with game rating: \"{}\"", rating);
    println!("I was called with game video_url: \"{}\"", video_url);
    println!("I was called with game img_url: \"{}\"", img_url);
    println!("I was called with game exe_url: \"{}\"", exe_url);
    

     // Load the MongoDB connection string from an environment variable:
     let client_uri =
     env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

 // A Client is needed to connect to MongoDB:
 // An extra line of code to work around a DNS issue on Windows:
//  let options =
//      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("error");
//  let client = mongodb::Client::with_options(options).unwrap();


    // search mongo db with cursor for game name
    
    // when found, update all info
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
