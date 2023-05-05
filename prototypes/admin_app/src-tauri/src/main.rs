// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update_entry(id: i16, name: &str, description: &str, year: &str, rating: &str, video_url: &str, img_url: &str, exe_url: &str) {
    println!("I was called with game name: \"{}\"", name);
    println!("I was called with game description: \"{}\"", description);
    println!("I was called with game year: \"{}\"", year);
    println!("I was called with game rating: \"{}\"", rating);
    println!("I was called with game video_url: \"{}\"", video_url);
    println!("I was called with game img_url: \"{}\"", img_url);
    println!("I was called with game exe_url: \"{}\"", exe_url);
    

    // search mongo db with cursor for game name
    
    // when found, update all info
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
