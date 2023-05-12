// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};
use std::process::Command;
use std::{fs, thread, time::Duration, path::Path};
//use std::path::PathBuf;

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

// ================== ./data bad need to call rust stuff like discord message
// add arg for string and replace ./data
fn download_all(app: tauri::AppHandle) -> bool
{   
    let download_path = app
        .path_resolver()
        .app_local_data_dir()
        .expect("failed to resolve resource dir")
        .as_os_str()
        .to_str()
        .unwrap();

    println!("{}", (&f!("{download_path} Is the path windows gave us for download_path")));

    // check if data folder exists
    if Path::new(download_path).is_dir() == true 
    {   
        println!("{}", (&f!("{download_path} path does exist")));
        
        // check if large_files folder !exists
        if Path::new(&f!("{download_path}/large_files")).is_dir() != true 
        { 
            // TODO add check for file version
            println!("but large files doesn't exist");
            update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
            println!("Download, done!");
            mv_and_unzip("tmp.zip", &f!("{download_path}"));
        }
    } 
    else
    {
        println!("{}", &f!("{download_path} doesn't exist"));
        fs::create_dir(&f!("{download_path}")).expect(&f!("failed to make {download_path} dir"));
        update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", &f!("{download_path}\\tmp.zip"));
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
    return download_all(app); //change to accept &str arg for path
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
    .setup(|app| {
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![close_splash, update_entry])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
