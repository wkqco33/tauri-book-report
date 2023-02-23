use std::error::Error;

use rusqlite::{Connection, Result};

pub struct BookReportDB {
    conn: Connection,
}

impl BookReportDB {
    pub fn init_database() -> Result<Connection, Box<dyn Error + Send + Sync>> {
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

        Ok(conn)
    }
}
