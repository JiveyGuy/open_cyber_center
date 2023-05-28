// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};
use std::process::Command;
use std::{fs, thread, time::Duration, path::Path, env, vec};
use bson::{doc, oid::ObjectId};
use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use mongodb::{options::{ClientOptions, ResolverConfig}};
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate fstrings;

/*
    Structs!!
    the payload type must implement `Serialize` and `Clone`.
*/
#[derive(Clone, serde::Serialize)]
struct Payload
{
  message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameStruct 
{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    name: String,
    description: String,
    year: String,
    rating: String,
    video_url: String,
    img_url: String,
}

// #[derive(Serialize, Deserialize)]
// struct MongoAuth {
//     MONGODB_URI : String,  
// }

#[derive(Serialize, Deserialize, Debug)]
struct FilterOnIdStruct 
{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
}

// fn get_mongo_auth(data_dir:&str) -> FirebaseLogin
// {
//     let file_path: &str = "./src/occ_firebase_login.json";
//     let the_file: String = fs::read_to_string(file_path)
//         .expect("Failed to read file_path for firebase auth.");
//     let auth_info:FirebaseLogin = serde_json::from_str(the_file.as_str()).expect("JSON was not well-formatted");
//     return auth_info;
// }


async fn get_mongo_handle(data_dir:&str) -> mongodb::Database
{
    // Load the MongoDB connection string from an environment variable:
    // let auth_file_location = f!("{data_dir}/mongo_auth.json").as_str();

    let client_uri: String = "mongodb+srv://dev_jivey_pc:14589@opencybercenter.hxek1qk.mongodb.net/?retryWrites=true&w=majority".to_string();

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options: ClientOptions = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("error");
    let client: mongodb::Client = mongodb::Client::with_options(options).unwrap();

    // Get the 'movies' collection from the 'sample_mflix' database:
    let db: mongodb::Database = client.database("test");
    return db;
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn update_entry(app: tauri::AppHandle, id: &str, name: &str, description: &str, year: &str, rating: &str, video_url: &str, img_url: &str, exe_url: &str) -> Result<String, ()> 
{
    // connect to mongo
    let data_dir = data_dir(app);
    let db: mongodb::Database = get_mongo_handle(data_dir.as_str()).await;
    
    // Create a cursor to search for the game ID
    let collection = db.collection::<GameStruct>("GameList");
    
    let filter_struct = FilterOnIdStruct { 
        _id: Some(ObjectId::parse_str(id).expect("Failed to parse as object ID!"))
    };
    
    //  println!("Object id = {}", ); //todo
    let filter = doc! { "_id": filter_struct._id };

    let update = doc! 
    {
        "$set" : {
        "name": name,
        "description": description,
        "year": year,
        "rating": rating,
        "video_url": video_url,
        "img_url": img_url,
        "exe_url": exe_url,
        }
    };
    
    match collection
    .find_one_and_update(filter, update, None).await {
        Ok(_res) => {
            println!("Updated one");
        },
        
        Err(e) => {
            println!("{}", e);
            println!("Error");
        }      
    }
    
    return Ok(format!("Ok"));
}


async fn write_games_json(data_dir:&str) -> std::io::Result<()> 
{

    let path_to_save = f!("{data_dir}/games.json");

    let db: mongodb::Database = get_mongo_handle(data_dir).await;
    let games: mongodb::Collection<GameStruct> = db.collection::<GameStruct>("GameList");
    let cursor: mongodb::Cursor<GameStruct> = games.find(None, None).await.expect("error");
    let vec_cursor: Vec<GameStruct> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);  
    
    // ======== Make output string text
    let mut buffer_string: String = String::new();
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


// Input: gid, out path including fiel name
fn update_local_resources(g_id: &str, out_str: &str)
{
    let output = Command::new("gdown")
        .arg(g_id)
        .arg("-O")
        .arg(out_str)
        .output()
        .expect("Failed to run command.");
    println!("{}", output.status);  
}

// call with where the file is and the path to it's resting dir
// example: mv_and_unzip("tmp.zip", "tmp")
fn mv_and_unzip(loc_start: &str, loc_end: &str)
{
    
    let resting_path: &str = &f!("{loc_end}/tmp.zip");
    println!("{}",f!("resting_path = {resting_path}"));
    
    if std::path::Path::new(resting_path).exists()
    {
        fs::remove_file(resting_path)
            .expect("Failed to remove goal file.");
    }   
    // mv 
    fs::rename(loc_start, resting_path)
        .expect("Failed to move file.");

    let _output = Command::new("tar")
        .arg("-xf")
        .arg(resting_path)
        .arg("-C")
        .arg(loc_end)
        .output()
        .expect("Failed to run command.");
}
#[tauri::command(async)]
fn get_data_dir(app: tauri::AppHandle) -> String
{
    return data_dir(app);
}

fn data_dir(app: tauri::AppHandle) -> String
{
    let download_path = app
        .path_resolver()
        .app_local_data_dir()
        .expect("failed to resolve resource dir");

    let download_path: String = download_path
        .as_os_str()
        .to_string_lossy()
        .to_string();

    return download_path;
}

// ================== ./data bad need to call rust stuff like discord message
// add arg for string and replace ./data
async fn download_all(app: tauri::AppHandle) -> bool
{   
    
    let download_path = data_dir(app);
    let download_path = download_path.as_str();

    write_games_json(download_path)
        .await
        .expect("Failed to write games.json");

    println!("{}", (&f!("{download_path} Is the path windows gave us for download_path")));

    // check if data folder exists
    if Path::new(&download_path).is_dir() == true 
    {   
        println!("{}", (&f!("{download_path} path does exist")));
        
        // check if large_files folder !exists
        if Path::new(&f!("{download_path}/large_files")).is_dir() != true 
        { 
            // TODO add check for file version
            println!("but large files doesn't exist");
            update_local_resources("1Bi4R6z7tBf6ugWib3_FztTv-mG61OuYU", "tmp.zip");
            println!("Download, done!");
            mv_and_unzip("tmp.zip", &f!("{download_path}"));
        }
    } 
    else
    {
        println!("{}", &f!("{download_path} doesn't exist"));
        fs::create_dir(&f!("{download_path}")).expect(&f!("failed to make {download_path} dir"));
        update_local_resources("1Bi4R6z7tBf6ugWib3_FztTv-mG61OuYU", &f!("{download_path}\\tmp.zip"));
        println!("Download, done!");
        mv_and_unzip(&f!("{download_path}/tmp.zip"), &f!("{download_path}/large_files"));
        
    }
    
    // change to src/assets for build
    println!("{}", (&f!("download_all ran, returning true")));

    return true;
}


// ============ COMMANDS EXPOSED
#[tauri::command(async)]
async fn close_splash (window: tauri::Window, app: tauri::AppHandle) -> bool{ // add arg for app handle
  
    thread::sleep(Duration::from_secs(3));
    if let Some(splash) = window.get_window("splash_window")
    {
        splash.close().unwrap();
    }

    thread::sleep(Duration::from_secs(2));
    if let Some(main_window) =  window.get_window("main_widow")
    {
        main_window.show().unwrap();
    }

    println!("{}", f!("close_splash ran, starting downloads"));

    // Impliment simong changes to find temp path

    // after getting temp_data_dir print it
    //     &str not String
    // println!("{}", f!("Temp_data_dir = {temp_data_dir}"));
    return download_all(app).await; //change to accept &str arg for path
}

#[tauri::command]
fn run_game(name: &str) {
    println!("{}", f!("run game called with {name}"));
    
    let _output = Command::new(name)
        .spawn()
        .expect("failed to execute process");
}

fn main()
{
    tauri::Builder::default()
    .setup(|app| {
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![close_splash, run_game, get_data_dir])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
