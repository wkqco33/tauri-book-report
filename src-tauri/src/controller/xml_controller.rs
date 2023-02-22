use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::error::Error;

use super::data_controller::BookInfo;

pub async fn xml_parser(xml: &str) -> Result<Vec<BookInfo>, Box<dyn Error + Send + Sync>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut book_data: Vec<BookInfo> = Vec::new();
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        let mut book_info: BookInfo;

        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,

            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"row" => {
                        
                    },
                    b"STD_YM" => {
                        book_info.year = txt.pop().unwrap();
                    }
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }

    Ok(book_data)
}
