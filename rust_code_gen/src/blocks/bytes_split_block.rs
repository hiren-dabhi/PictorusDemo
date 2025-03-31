use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use log::debug;

use crate::{
    block_data::{BlockData, BlockDataType},
    byte_data::{find_all_bytes_idx, parse_string_to_read_delimiter},
    stale_tracker::StaleTracker,
    traits::IsValid,
};
use utils::{buffer_to_scalar, parse_select_spec};

pub struct BytesSplitBlock {
    pub name: &'static str,
    pub delimiter: (Vec<u8>, Vec<usize>, usize),
    pub select_data: Vec<(BlockDataType, usize)>,
    pub data: Vec<BlockData>,
    pub stale_check: StaleTracker,
}

impl BytesSplitBlock {
    pub fn new(
        name: &'static str,
        delimiter: &str,
        select_data: &[String],
        stale_age_ms: f64,
    ) -> Self {
        let mut select_data = parse_select_spec(select_data);
        let data = if !select_data.is_empty() {
            select_data
                .iter()
                .map(|(dt, _)| match dt {
                    BlockDataType::BytesArray => BlockData::from_bytes(b""),
                    _ => BlockData::from_scalar(0.0),
                })
                .collect()
        } else {
            // Default to trying to parse single value as a scalar
            select_data.push((BlockDataType::Scalar, 0));
            vec![BlockData::from_scalar(0.0)]
        };

        let delimiter = parse_string_to_read_delimiter(delimiter);
        let delimiter_len = delimiter.0.len() + delimiter.1.len();
        let delimiter = (delimiter.0, delimiter.1, delimiter_len);

        BytesSplitBlock {
            name,
            delimiter,
            select_data,
            data,
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }
    pub fn run(&mut self, input: &BlockData, app_time_s: f64) {
        let res = match input.get_type() {
            BlockDataType::BytesArray => self.parse_bytes(input),
            _ => panic!("BytesSplitBlock only supports byte data"),
        };

        if res.is_ok() {
            self.stale_check.mark_updated(app_time_s);
        }
    }

    fn parse_bytes(&mut self, input: &BlockData) -> Result<(), ()> {
        let bytes_data = input.to_bytes();
        debug!(
            "{}: Splitting data: {:?} on delimiter: {:?}",
            self.name, bytes_data, self.delimiter
        );
        let delim_indices = find_all_bytes_idx(&bytes_data, &self.delimiter.0, &self.delimiter.1);

        for (i, (dt, data_idx)) in self.select_data.iter().enumerate() {
            let data_idx = *data_idx;
            if delim_indices.len() < data_idx {
                return Err(());
            }

            let chunk_start = if data_idx == 0 {
                0
            } else {
                delim_indices[data_idx - 1] + self.delimiter.2
            };

            let chunk_end = if data_idx == delim_indices.len() {
                bytes_data.len()
            } else {
                delim_indices[data_idx]
            };

            match dt {
                BlockDataType::Scalar => {
                    self.data[i].set_scalar(buffer_to_scalar(&bytes_data[chunk_start..chunk_end])?);
                }
                BlockDataType::BytesArray => {
                    self.data[i].set_bytes(&bytes_data[chunk_start..chunk_end]);
                }
                _ => {
                    debug!("{}: Unsupported data type {:?}", self.name, dt);
                    return Err(());
                }
            };

            debug!("{}: Value {} = {:?}", self.name, i, self.data[i],);
        }
        Ok(())
    }
}

impl IsValid for BytesSplitBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splits_and_parses_data() {
        let mut block = BytesSplitBlock::new(
            "foo",
            ":",
            &["Scalar:0.0".into(), "BytesArray:2".into()],
            1000.0,
        );
        let input = BlockData::from_bytes(br#"123:4.56:78.9"#);
        block.run(&input, 0.1);
        assert_eq!(
            block.data,
            vec![BlockData::from_scalar(123.), BlockData::from_bytes(b"78.9")]
        );
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_splits_and_parses_empty_input() {
        let mut block = BytesSplitBlock::new(
            "foo",
            ":",
            &["Scalar:0.0".into(), "BytesArray:2".into()],
            1000.0,
        );
        let input = BlockData::from_bytes(b"");
        block.run(&input, 0.1);
        assert_eq!(
            block.data,
            vec![BlockData::from_scalar(0.0), BlockData::from_bytes(b"")]
        );
        assert!(!block.is_valid(0.1).all());
    }

    #[test]
    fn test_splits_and_parses_no_delimiters() {
        let mut block = BytesSplitBlock::new(
            "foo",
            ":",
            &["Scalar:0.0".into(), "BytesArray:1".into()],
            1000.0,
        );
        let input = BlockData::from_bytes(b"123456789");
        block.run(&input, 0.1);
        assert_eq!(
            block.data,
            vec![
                BlockData::from_scalar(123456789.0),
                BlockData::from_bytes(b"")
            ]
        );
        assert!(!block.is_valid(0.1).all());
    }

    #[test]
    fn test_splits_and_parses_wildcard_delimiters() {
        let mut block = BytesSplitBlock::new(
            "foo",
            r"\xAA\x**\xAB",
            &["BytesArray:0.0".into(), "BytesArray:2".into()],
            1000.0,
        );
        let input = BlockData::from_bytes(b"\x00\xAA\xAA\xAB\x01\xAA\xFF\xAB\x02");
        block.run(&input, 0.1);
        assert_eq!(
            block.data,
            vec![
                BlockData::from_bytes(b"\x00"),
                BlockData::from_bytes(b"\x02")
            ]
        );
        assert!(block.is_valid(0.1).all());
    }
}
