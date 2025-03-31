use alloc::vec::Vec;

use crate::{block_data::BlockData, byte_data::parse_string_to_bytes};
use log::debug;

pub struct BytesJoinBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub delimiter: Vec<u8>,
}

impl BytesJoinBlock {
    pub fn new(name: &'static str, _: &BlockData, delimiter: &str) -> BytesJoinBlock {
        BytesJoinBlock {
            name,
            data: BlockData::from_bytes(b""),
            delimiter: parse_string_to_bytes(delimiter),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        let entries: Vec<Vec<u8>> = inputs.iter().map(|input| input.to_bytes()).collect();

        let delimited_data = entries.join(self.delimiter.as_slice());
        self.data.set_bytes(&delimited_data);

        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;

    #[test]
    fn test_bytes_join_block() {
        let delimiter: &str = "/ ";
        let mut block = BytesJoinBlock::new("BytesJoin1", &BlockData::from_bytes(b""), delimiter);

        // First input to the block is a scalar
        let signal1 = BlockData::from_scalar(1.0);

        // Second input is a vector
        let signal2 = BlockData::from_vector(&[2.5, 3.14159]);

        // Third input is some bytes data
        let signal3 = BlockData::from_bytes(b"hello there");

        block.run(&[&signal1, &signal2, &signal3]);

        let expected_string = "1.0/ [[2.5,3.14159]]/ hello there".to_string();

        assert_eq!(block.data.raw_string(), expected_string)
    }

    #[test]
    fn test_bytes_join_block_empty_input() {
        let delimiter: &str = "/ ";
        let mut block =
            BytesJoinBlock::new("BytesJoinEmpty", &BlockData::from_bytes(b""), delimiter);
        block.run(&[]);

        let expected_string = "".to_string();
        assert_eq!(block.data.raw_string(), expected_string);
    }

    #[test]
    fn test_bytes_join_block_non_ascii() {
        let delimiter: &str = "⚡";
        let mut block = BytesJoinBlock::new("BytesJoin3", &BlockData::from_bytes(b""), delimiter);

        let signal1 = BlockData::from_bytes("привет".as_bytes());
        let signal2 = BlockData::from_bytes("こんにちは".as_bytes());

        block.run(&[&signal1, &signal2]);

        let expected_string = "привет⚡こんにちは".to_string();
        assert_eq!(block.data.raw_string(), expected_string);
    }
}
