use crate::{block_data::BlockData, byte_data::parse_string_to_bytes};
use alloc::vec::Vec;
use embedded_io::Write;
use log::debug;

pub struct SerialTransmitBlock {
    name: &'static str,
    start_delimiter: Vec<u8>,
    end_delimiter: Vec<u8>,
}

impl SerialTransmitBlock {
    pub fn new(name: &'static str, start_delimiter: &str, end_delimiter: &str) -> Self {
        SerialTransmitBlock {
            name,
            start_delimiter: parse_string_to_bytes(start_delimiter),
            end_delimiter: parse_string_to_bytes(end_delimiter),
        }
    }

    pub fn run(&mut self, input: &BlockData, protocol: &mut impl Write) {
        let write_val = [
            self.start_delimiter.as_slice(),
            input.to_bytes().as_slice(),
            self.end_delimiter.as_slice(),
        ]
        .concat();
        debug!("{}: Transmitting value: {:?}", self.name, &write_val);
        protocol.write(&write_val).ok();
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use mockall::predicate::eq;

    use protocols::MockWrite;

    use super::*;

    #[test]
    fn test_writes_expected_data() {
        let mut mock_proto = MockWrite::new();
        let expected = "42";
        mock_proto
            .expect_write()
            .with(eq(expected.as_bytes()))
            .return_once(|_| Ok(expected.len()));

        let mut block = SerialTransmitBlock::new("Foo", "", "");
        block.run(&BlockData::from_bytes(expected.as_bytes()), &mut mock_proto);
    }

    #[test]
    fn test_writes_delimited_data() {
        let mut mock_proto = MockWrite::new();
        let expected = "$GPGSA,1,2,3\r\n";
        mock_proto
            .expect_write()
            .with(eq(expected.as_bytes()))
            .return_once(|_| Ok(expected.len()));

        let mut block = SerialTransmitBlock::new("Foo", "$GPGSA,", "\r\n");
        block.run(&BlockData::from_bytes(b"1,2,3"), &mut mock_proto);
    }
}
