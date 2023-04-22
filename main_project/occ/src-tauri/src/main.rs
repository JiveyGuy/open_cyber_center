// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
//   )]

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// // fn greet(name: &str) -> String {
// //     format!("Hello, {}! You've been greeted from Rust!", name)
// // }

// fn main() {
//     tauri::Builder::default()
//         /*
//         .invoke_handler(tauri::generate_handler![greet])
//         */
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


#[cfg(not(feature = "ui"))]
mod rust {
  use std::{thread::sleep, time::Duration};
  use tauri::Manager;

  // this command is here just so the example doesn't throw an error
  #[tauri::command]
  fn close_splashscreen() {}

  pub fn main() {
    tauri::Builder::default()
      .setup(|app| {
        let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_window("main").unwrap();
        // we perform the initialization code on a new task so the app doesn't freeze
        tauri::async_runtime::spawn(async move {
          println!("Initializing...");
          sleep(Duration::from_secs(2));
          println!("Done initializing.");

          // After it's done, close the splashscreen and display the main window
          splashscreen_window.close().unwrap();
          main_window.show().unwrap();
        });
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![close_splashscreen])
      .run(tauri::generate_context!())
      .expect("failed to run app");
  }
}

// Application code for a splashscreen system that waits for the UI
#[cfg(feature = "ui")]
mod ui {
  use std::sync::{Arc, Mutex};
  use tauri::{Manager, Params, State, Window};

  // wrappers around each Window
  // we use a dedicated type because Tauri can only manage a single instance of a given type
  struct SplashscreenWindow<P: Params>(Arc<Mutex<Window<P>>>);
  struct MainWindow<P: Params>(Arc<Mutex<Window<P>>>);

  #[tauri::command]
  fn close_splashscreen<P: Params>(
    _: Window<P>, // force inference of P
    splashscreen: State<SplashscreenWindow<P>>,
    main: State<MainWindow<P>>,
  ) {
    // Close splashscreen
    splashscreen.0.lock().unwrap().close().unwrap();
    // Show main window
    main.0.lock().unwrap().show().unwrap();
  }

  pub fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());


    tauri::Builder::default()
      .plugin(tauri_plugin_localhost::Builder::new(port).build())
      .setup(move |app| {
        // set the splashscreen and main windows to be globally available with the tauri state API
        app.manage(SplashscreenWindow(Arc::new(Mutex::new(
          app.get_window("splashscreen").unwrap(),
        ))));
        app.manage(MainWindow(Arc::new(Mutex::new(
          app.get_window("main").unwrap(),
        ))));
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![close_splashscreen])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
}

fn main() {
  #[cfg(feature = "ui")]
  ui::main();
  #[cfg(not(feature = "ui"))]
  rust::main();
}