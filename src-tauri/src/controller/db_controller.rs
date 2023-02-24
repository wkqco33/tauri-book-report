use std::error::Error;

use rusqlite::Connection;

use super::super::model::book_report::BookReport;

pub struct BookReportDB;

impl BookReportDB {
    pub fn init_database() {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS book_report (
                  id INTEGER AUTO_INCREMENT PRIMARY KEY,
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

    pub fn select_all_book_report() -> Result<Vec<BookReport>, Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        let mut stmt = conn.prepare("SELECT * FROM book_report")?;
        let book_report_iter = stmt
            .query_map([], |row| {
                Ok(BookReport::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                ))
            })
            .expect("Failed to select all book report");

        let book_report_vec = book_report_iter.map(|r| r.unwrap()).collect();

        Ok(book_report_vec)
    }

    pub fn select_book_report_by_id(id: i32) -> Result<BookReport, Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        let mut stmt = conn.prepare("SELECT * FROM book_report WHERE id = ?1")?;
        let book_report = stmt
            .query_row([], |row| {
                Ok(BookReport::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                ))
            })
            .expect("Failed to select book report by id");

        Ok(book_report)
    }

    pub fn insert_book_report(book_report: &BookReport) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute(
            "INSERT INTO book_report (title, book_name, author, start_date, end_date, favorite, description)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                book_report.get_title(),
                book_report.get_book_name(),
                book_report.get_author(),
                book_report.get_start_date(),
                book_report.get_end_date(),
                book_report.get_favorite(),
                book_report.get_description()
            ],
        )
        .expect("Failed to insert book report");

        Ok(())
    }

    pub fn update_book_report(book_report: BookReport) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute(
            "UPDATE book_report
                  SET title = ?1, book_name = ?2, author = ?3, start_date = ?4, end_date = ?5, favorite = ?6, description = ?7
                  WHERE id = ?8",
            rusqlite::params![
                book_report.get_title(),
                book_report.get_book_name(),
                book_report.get_author(),
                book_report.get_start_date(),
                book_report.get_end_date(),
                book_report.get_favorite(),
                book_report.get_description(),
                book_report.get_id()
            ],
        )
        .expect("Failed to update book report");

        Ok(())
    }

    pub fn delete_book_report(id: i32) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute(
            "DELETE FROM book_report WHERE id = ?1",
            rusqlite::params![id],
        )
        .expect("Failed to delete book report");

        Ok(())
    }

    pub fn delete_all_book_report() -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("book_report.db").expect("Failed to open database");

        conn.execute("DELETE FROM book_report", rusqlite::params![])
            .expect("Failed to delete all book report");

        Ok(())
    }
}
