// TODO: Chris owes everyone a beer if we don't fix this by Sept 24
use hex::FromHex;
/// #[cfg(not(feature = "sim"))]
use std::fs::File;
// // #[cfg(feature = "sim")]
use std::io::{BufRead, BufReader, Lines};

use utils::BlockData;

#[derive(strum::EnumString)]
pub enum ReadLogMethod {
    Default,
    Bytes,
}

pub struct ReadLogBlock {
    pub name: &'static str,
    // #[cfg(not(feature = "sim"))]
    lines: Lines<BufReader<File>>,
    // #[cfg(feature = "sim")]
    //lines: Lines<BufReader<Cursor<String>>>,
    pub data: BlockData,
    pub method: ReadLogMethod,
}

impl ReadLogBlock {
    pub fn new(name: &'static str, file_path: &str, method: &str) -> Self {
        log::debug!("{}: Creating ReadLogBlock at path {}", name, file_path);
        // // #[cfg(not(feature = "sim"))]
        let reader = {
            let file = File::open(file_path).expect("file not found");
            BufReader::new(file)
        };
        /*
        // #[cfg(feature = "sim")]
        let reader = BufReader::new(Cursor::new(String::from("")));
        */
        let lines = reader.lines();
        Self {
            name,
            lines,
            data: BlockData::from_bytes(b""),
            method: method.parse().unwrap(),
        }
    }
    pub fn run(&mut self) {
        log::debug!("{}: Running ReadLogBlock", self.name);
        let next_line = self.lines.next();
        match next_line {
            Some(line) => {
                let line = line.unwrap();
                let bytes = match self.method {
                    ReadLogMethod::Default => line.as_bytes().to_vec(),
                    ReadLogMethod::Bytes => Vec::from_hex(line).unwrap(),
                };
                self.data.set_bytes(&bytes);
            }
            None => {
                // TODO: What should we do here? Wrap around to the beginning of the file?
                log::debug!("End of file");
                panic!("End of file reached");
            }
        }
    }
}
