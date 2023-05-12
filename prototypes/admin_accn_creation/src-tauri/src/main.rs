// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Imports
use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::{env, fs, collections::HashMap};

#[derive(Serialize, Deserialize)]
struct FirebaseLogin {
    project_id                    : String,  
    private_key_id                : String,
    private_key                   : String,
    client_email                  : String,
    client_id                     : String,
    auth_uri                      : String,
    token_uri                     : String,
    auth_provider_x509_cert_url   : String,
    client_x509_cert_url          : String,
    universe_domain               : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
  name: String
}

// todo UID stuff

// binding mongo with UID

// safely load from local_env.json
fn get_fire_auth() -> FirebaseLogin
{
    let file_path: &str = "./src/occ_firebase_login.json";
    let the_file: String = fs::read_to_string(file_path)
        .expect("Failed to read file_path for firebase auth.");
    let auth_info:FirebaseLogin = serde_json::from_str(the_file.as_str()).expect("JSON was not well-formatted");
    return auth_info;
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn get_users() -> Vec<&'static str> 
{
    let auth_info: FirebaseLogin = get_fire_auth(); 
    let firebase = Firebase::auth(&auth_info.auth_uri, &auth_info.private_key).unwrap().at("users");
    // let firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap().with_params().start_at(1).order_by("name").equal_to(5).finish();

    // let users =  firebase.at("users").at("USER_ID")
    //             .get::<HashMap<String, User>>()
    //             .await;
    // let users = users.expect("Failed to get users.");
    // for (key, value) in &users {
    //     println!("{}: {}", key, value.name);
    // }
    let users = firebase
        .get::<HashMap<String, User>>()
        .await
        .expect("failed to get users.");

    return vec!["a","a","a","a","a","a"];
}

#[tauri::command]
fn add_user(name: &str) 
{
    println!("add_user called with  {}", name);
}
// https://github.com/emreyalvac/firebase-rs
// todo
fn main() 
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_user, get_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
