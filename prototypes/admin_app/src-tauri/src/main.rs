// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update_entry(name: &str, ) {
    println!("I was called with game name: \"{}\"", name);

    // search mongo db with cursor for game name
    
    // when found, update all info
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
