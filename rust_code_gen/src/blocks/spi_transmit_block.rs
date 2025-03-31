use crate::block_data::BlockData;
use log::debug;
use protocols::SpiProtocol;

pub struct SpiTransmitBlock {
    name: &'static str,
}

impl SpiTransmitBlock {
    pub fn new(name: &'static str) -> Self {
        SpiTransmitBlock { name }
    }

    pub fn run(&mut self, input: &BlockData, protocol: &mut impl SpiProtocol) {
        let write_val = input.to_bytes();
        debug!("{}: Transmitting value: {:?}", self.name, &write_val);
        protocol.write(&write_val).ok();
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use protocols::MockSpiProtocol;

    #[test]
    fn test_writes_expected_data() {
        let mut mock_proto = MockSpiProtocol::new();
        let tx_buff = [0x01, 0x02, 0x03, 0x04];

        mock_proto
            .expect_write()
            .withf(move |buf| buf == tx_buff)
            .return_once(|_| Ok(()));

        let mut block = SpiTransmitBlock::new("Foo");
        block.run(
            &BlockData::from_bytes(&[0x01, 0x02, 0x03, 0x04]),
            &mut mock_proto,
        );
    }
}
