// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};
use std::process::Command;
use std::{fs, thread, time::Duration, path::Path};



#[macro_use]
extern crate fstrings;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload
{
  message: String,
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

fn download_all() -> bool
{   
    // check if data folder exists
    if Path::new("./data").is_dir() == true 
    {   
        println!("./data does exist");
        // check if large_files folder !exists
        if Path::new("./data/large_files").is_dir() != true 
        { // TODO add check for file version
            println!("but large files doesn't exist");
            update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
            println!("Download, done!");
            mv_and_unzip("tmp.zip", "./data");
        }
    } 
    else
    {
        println!("./data doesn't exist");
        fs::create_dir("./data").expect("failed to make .data dir");
        update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", ".\\data\\tmp.zip");
        println!("Download, done!");
        mv_and_unzip("./data/tmp.zip", "./data/large_files");
    }
    
    // change to src/assets for build
    println!("{}", f!("download_all ran, returning true"));
    return true;
}

#[tauri::command(async)]
async fn close_splash (window: tauri::Window) -> bool{
  
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
    return download_all();
}

#[tauri::command]
fn update_entry(name: &str) {
    let _output = Command::new(name)
        .spawn()
        .expect("failed to execute process");
}

fn main()
{

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splash, update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
