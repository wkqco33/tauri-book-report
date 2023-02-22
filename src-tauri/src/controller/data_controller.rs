use reqwest::get;

use super::xml_controller::xml_parser;

const SERVICE_KEY: &str = include_str!("key.txt");

#[derive(Debug)]
pub struct BookInfo {
    pub year: String,
    pub rank: String,
    pub book_name: String,
    pub author: String,
    pub publisher: String,
    pub publication_year: String,
    pub volume: String,
    pub image_url: String,
}

impl BookInfo {
    pub fn new(
        year: String,
        rank: String,
        book_name: String,
        author: String,
        publisher: String,
        publication_year: String,
        volume: String,
        image_url: String,
    ) -> Self {
        BookInfo {
            year,
            rank,
            book_name,
            author,
            publisher,
            publication_year,
            volume,
            image_url,
        }
    }
}

pub async fn get_rank_data(index: &str) -> Result<Vec<BookInfo>, Box<dyn std::error::Error>> {
    let base_url = "https://openapi.gg.go.kr/Poplitloanbook";
    let data_type = "xml";
    let service_key = SERVICE_KEY;
    let page_index = index;
    let page_size = "100";

    let url = format!(
        "{base_url}?Type={data_type}&KEY={service_key}&pIndex={page_index}&pSize{page_size}",
        base_url = base_url,
        data_type = data_type,
        service_key = service_key,
        page_index = page_index,
        page_size = page_size,
    );

    print!("{:?}", url);

    let resp = get(&url).await?.text().await.expect("Failed to get text");
    let book_info_vec = xml_parser(&resp).await.expect("Failed to parse xml");

    Ok(book_info_vec)
}
