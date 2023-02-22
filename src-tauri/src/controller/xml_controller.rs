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
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"row" => {
                    print!("row: ");
                }
                b"STD_YM" => {
                    let year = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"RKI_NO" => {
                    let rank = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"BOOK_NM_INFO" => {
                    let book_name = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"AUTHOR_NM_INFO" => {
                    let author = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"PUBLSHCMPY_NM" => {
                    let publisher = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"PUBLCATN_YY" => {
                    let publication_year = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"VOLM_CNT" => {
                    let volume = reader.read_text(e.name()).unwrap().into_owned();
                }
                b"BOOK_IMAGE_URL" => {
                    let image_url = reader.read_text(e.name()).unwrap().into_owned();
                },
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        book_data.push(BookInfo::new(
            txt[0].to_string(),
            txt[1].clone(),
            txt[2].clone(),
            txt[3].clone(),
            txt[4].clone(),
            txt[5].clone(),
            txt[6].clone(),
            txt[7].clone(),
        ));
        buf.clear();
    }
    Ok(book_data)
}
