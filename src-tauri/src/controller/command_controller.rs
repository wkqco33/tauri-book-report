use crate::model::book_report::BookReport;

use super::super::BookInfo;
use super::data_controller::get_book_rank_data;
use super::db_controller::BookReportDB;

#[tauri::command]
pub async fn request_rank_data(page: i32) -> Result<Vec<BookInfo>, String> {
    let book_data = get_book_rank_data(&page.to_string(), "50").await;
    let mut book_data = book_data.expect("Error while getting book data");
    book_data.sort_by(|a, b| a.rank.cmp(&b.rank));

    Ok(book_data)
}

#[tauri::command]
pub fn request_all_report_data() -> Result<Vec<BookReport>, String> {
    let book_report_data = BookReportDB::select_all_book_report();

    match book_report_data {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub fn request_report_data(id: i32) -> Result<BookReport, String> {
    let book_report_data = BookReportDB::select_book_report_by_id(&id);

    match book_report_data {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub fn request_save_report(book_report: BookReport) -> Result<bool, bool> {
    let result = BookReportDB::insert_book_report(&book_report);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

#[tauri::command]
pub fn request_update_report(book_report: BookReport) -> Result<bool, bool> {
    let result = BookReportDB::update_book_report(&book_report);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

#[tauri::command]
pub fn request_delete_report(id: i32) -> Result<bool, bool> {
    let result = BookReportDB::delete_book_report(&id);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}
