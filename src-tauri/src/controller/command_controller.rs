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
