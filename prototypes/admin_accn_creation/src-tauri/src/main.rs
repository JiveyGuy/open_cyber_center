// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn gest_users() -> Vec<&'static str> {
    println!("get_users called");
    return vec!["a","a","a","a","a","a"];
}

#[tauri::command]
fn add_user(name: &str) {
    println!("add_user called with  {}", name);
}
// https://github.com/emreyalvac/firebase-rs
// todo
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_user, gest_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
