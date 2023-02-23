#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod model;

use controller::command_controller::request_rank_data;
use controller::db_controller::BookReportDB;
use model::book_info::BookInfo;
use tauri::Manager;

fn main() {
    BookReportDB::init_database();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![request_rank_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
