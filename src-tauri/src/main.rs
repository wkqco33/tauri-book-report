#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod model;

use controller::command_controller::request_rank_data;
use controller::db_controller::BookReportDB;
use model::book_info::BookInfo;

fn main() {

    match BookReportDB::init_database() {
        Ok(_) => println!("Database initialized"),
        Err(_) => println!("Failed to initialize database"),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request_rank_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
