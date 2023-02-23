#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BookInfo {
    year: String,
    rank: String,
    book_name: String,
    author: String,
    publisher: String,
    publication_year: String,
    volume: String,
    image_url: String,
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