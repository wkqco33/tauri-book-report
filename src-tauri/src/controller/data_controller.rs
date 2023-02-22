use reqwest::get;

const SERVICE_KEY: &str = include_str!("key.txt");

pub struct BookInfo {
    pub title: String,
    pub year: String,
    pub rank: String,
    pub book_name: String,
    pub author: String,
    pub publisher: String,
    pub publication_date: String,
    pub volume: String,
    pub image_url: String,
}

impl BookInfo {
    pub fn new(
        title: String,
        year: String,
        rank: String,
        book_name: String,
        author: String,
        publisher: String,
        publication_date: String,
        volume: String,
        image_url: String,
    ) -> Self {
        BookInfo {
            title,
            year,
            rank,
            book_name,
            author,
            publisher,
            publication_date,
            volume,
            image_url,
        }
    }
}

pub async fn get_rank_data(index: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = "https://openapi.gg.go.kr/Poplitloanbook";
    let data_type = "";
    let service_key = SERVICE_KEY;
    let page_index = index;
    let page_size = "50";

    let url = format!(
        "{base_url}?Type={data_type}/KEY={service_key}/pIndex={page_index}/pSize{page_size}",
        base_url = base_url,
        data_type = data_type,
        service_key = service_key,
        page_index = page_index,
        page_size = page_size,
    );

    let resp = get(&url).await?.text().await.expect("Failed to get text");
    print!("{:?}", resp);

    Ok(resp)
}
