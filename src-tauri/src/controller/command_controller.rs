use super::super::BookInfo;
use super::data_controller::get_book_rank_data;

#[tauri::command]
pub async fn request_rank_data(page: i32) -> Result<Vec<BookInfo>, String> {
    let book_data = get_book_rank_data(&page.to_string(), "50").await;

    match book_data {
        Ok(data) => {
            Ok(data)
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }
}