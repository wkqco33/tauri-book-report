#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;

use controller::data_controller::get_rank_data;

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    let book_data = get_rank_data("1").await;

    match book_data {
        Ok(data) => {
            data.iter().for_each(|book| {
                println!("{:?}", book);
            });
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
