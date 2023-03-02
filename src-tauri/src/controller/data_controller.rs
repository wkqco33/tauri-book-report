use reqwest::get;

use super::super::BookInfo;
use super::xml_paser::xml_parser;

const SERVICE_KEY: &str = include_str!("key.txt");

pub async fn get_book_rank_data(
    index: &str,
    size: &str,
) -> Result<Vec<BookInfo>, Box<dyn std::error::Error>> {
    let base_url = "https://openapi.gg.go.kr/Poplitloanbook";
    let data_type = "xml";
    let service_key = SERVICE_KEY;
    let page_index = index;
    let page_size = size;

    let url = format!(
        "{base_url}?Type={data_type}&KEY={service_key}&pIndex={page_index}&pSize{page_size}",
        base_url = base_url,
        data_type = data_type,
        service_key = service_key,
        page_index = page_index,
        page_size = page_size,
    );

    let resp = get(&url).await;
    let mut book_info_vec: Vec<BookInfo> = Vec::new();

    match resp {
        Ok(resp) => {
            let xml = resp.text().await.expect("Failed to get xml text");
            book_info_vec = xml_parser(&xml).await.expect("Failed to parse xml");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(book_info_vec)
}
