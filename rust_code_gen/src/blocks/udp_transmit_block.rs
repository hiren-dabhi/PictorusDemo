use log::{info, warn};

use crate::block_data::BlockData;
use protocols::UdpProtocol;

pub struct UdpTransmitBlock {
    pub name: &'static str,
    pub to_addr: String,
}

impl UdpTransmitBlock {
    pub fn new(name: &'static str, to_addr: &str) -> Self {
        info!("{}: Establishing UDP transmit socket...", name);
        UdpTransmitBlock {
            name,
            to_addr: to_addr.to_string(),
        }
    }

    pub fn run(&mut self, input: &BlockData, protocol: &mut dyn UdpProtocol) {
        if let Err(e) = protocol.write(&input.to_bytes(), &self.to_addr) {
            warn!("{}: Failed to write UDP data: {}", self.name, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use protocols::MockUdpProtocol;

    #[test]
    fn test_writes_expected_data() {
        let to_addr = "1.2.3.4:9001";
        let mut mock_proto = MockUdpProtocol::new();
        let expected = "42";
        mock_proto
            .expect_write()
            .with(eq(expected.as_bytes()), eq(to_addr))
            .return_once(|_, _| Ok(4));

        let mut block = UdpTransmitBlock::new("Foo", to_addr);
        let data = BlockData::from_bytes(expected.as_bytes());
        block.run(&data, &mut mock_proto);
    }
}
