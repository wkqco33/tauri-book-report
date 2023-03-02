#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BookReport {
    id: i32,
    title: String,
    book_name: String,
    author: String,
    start_date: String,
    end_date: String,
    publisher: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl BookReport {
    pub fn new(
        id: i32,
        title: String,
        book_name: String,
        author: String,
        start_date: String,
        end_date: String,
        publisher: String,
        description: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        BookReport {
            id,
            title,
            book_name,
            author,
            start_date,
            end_date,
            publisher,
            description,
            created_at,
            updated_at,
        }
    }

    pub fn new_empty() -> Self {
        BookReport {
            id: 0,
            title: "".to_string(),
            book_name: "".to_string(),
            author: "".to_string(),
            start_date: "".to_string(),
            end_date: "".to_string(),
            publisher: "".to_string(),
            description: "".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_book_name(&self) -> String {
        self.book_name.clone()
    }

    pub fn get_publisher(&self) -> String {
        self.publisher.clone()
    }

    pub fn get_author(&self) -> String {
        self.author.clone()
    }

    pub fn get_start_date(&self) -> String {
        self.start_date.clone()
    }

    pub fn get_end_date(&self) -> String {
        self.end_date.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}
