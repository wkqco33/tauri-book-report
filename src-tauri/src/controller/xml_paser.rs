use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::error::Error;

use super::super::BookInfo;

pub async fn xml_parser(xml: &str) -> Result<Vec<BookInfo>, Box<dyn Error + Send + Sync>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut book_data: Vec<BookInfo> = Vec::new();
    let mut info: HashMap<String, String> = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"row" => {
                    if info.len() > 0 {
                        info.clear();
                    }
                }
                b"STD_YM" => {
                    info.insert(
                        "STD_YM".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"RKI_NO" => {
                    info.insert(
                        "RKI_NO".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"BOOK_NM_INFO" => {
                    info.insert(
                        "BOOK_NM_INFO".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"AUTHOR_NM_INFO" => {
                    info.insert(
                        "AUTHOR_NM_INFO".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"PUBLSHCMPY_NM" => {
                    info.insert(
                        "PUBLSHCMPY_NM".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"PUBLCATN_YY" => {
                    info.insert(
                        "PUBLCATN_YY".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"VOLM_CNT" => {
                    info.insert(
                        "VOLM_CNT".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                b"BOOK_IMAGE_URL" => {
                    info.insert(
                        "BOOK_IMAGE_URL".to_string(),
                        reader.read_text(e.name()).unwrap().into_owned(),
                    );
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"row" => {
                    book_data.push(BookInfo::new(
                        info["STD_YM"].clone(),
                        info["RKI_NO"].clone(),
                        info["BOOK_NM_INFO"].clone(),
                        info["AUTHOR_NM_INFO"].clone(),
                        info["PUBLSHCMPY_NM"].clone(),
                        info["PUBLCATN_YY"].clone(),
                        info["VOLM_CNT"].clone(),
                        info["BOOK_IMAGE_URL"].clone(),
                    ));
                    info.clear();
                }
                _ => (),
            },
            _ => (),
        }
        buf.clear();
    }
    Ok(book_data)
}
