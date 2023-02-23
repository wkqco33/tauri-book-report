use rusqlite::Connection;

pub struct BookReportDB;

impl BookReportDB {
    pub fn init_database() {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS book_report (
                  id INTEGER PRIMARY KEY,
                  title TEXT NOT NULL,
                  book_name TEXT NOT NULL,
                  author TEXT,
                  start_date TEXT,
                  end_date TEXT,
                  favorite INTEGER,
                  description TEXT
                  )",
            (),
        )
        .expect("Failed to create table");
    }
}
