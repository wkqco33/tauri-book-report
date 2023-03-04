#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod model;

use std::mem::MaybeUninit;

use controller::command_controller::*;
use controller::db_controller::BookReportDB;
use model::book_info::BookInfo;
use tauri::Manager;

 static mut BOOK_REPORT_DB: MaybeUninit<BookReportDB> = MaybeUninit::uninit();

fn main() {
    unsafe {
        BOOK_REPORT_DB = MaybeUninit::new(BookReportDB::new());
        BOOK_REPORT_DB.assume_init_ref().init_database();
    }

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            request_rank_data,
            request_all_report_data,
            request_report_data,
            request_save_report,
            request_update_report,
            request_delete_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
