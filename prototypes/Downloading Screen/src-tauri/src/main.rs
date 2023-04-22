// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use std::process::Command;
use std::{fs, thread, time::Duration};

#[macro_use]
extern crate fstrings;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Input: gid, out path including fiel name
fn update_local_resources(g_id: &str, out_str: &str) {
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
fn mv_and_unzip(loc_start: &str, loc_end: &str) {
    
    let resting_path: &str = &f!("{loc_end}/tmp.zip");
    println!("{}",f!("resting_path = ./{resting_path}"));
    
    if std::path::Path::new(resting_path).exists() {
        fs::remove_file(resting_path)
            .expect("Failed to remove goal file.");
    }   
   
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

fn download_all() -> bool{
    update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
    println!("Download, done!");
    mv_and_unzip("tmp.zip", "../src/assets/");
    println!("{}", f!("download_all ran, returning true"));
    return true;
}

#[tauri::command(async)]
async fn close_splash (window: tauri::Window) -> bool{
  
    thread::sleep(Duration::from_secs(5));
    if let Some(splash) = window.get_window("splash_window") {
        splash.close().unwrap();
    }
    thread::sleep(Duration::from_secs(2));
    if let Some(main_window) =  window.get_window("main_window") {
        main_window.show().unwrap();
    }
    println!("{}", f!("close_splash ran, starting downloads"));
    return download_all();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
