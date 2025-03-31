use log::debug;

use crate::block_data::BlockData;
use crate::byte_data::BUFF_SIZE_BYTES;
use crate::stale_tracker::StaleTracker;
use crate::traits::IsValid;
use protocols::UdpProtocol;

pub struct UdpReceiveBlock {
    name: &'static str,
    pub data: BlockData,
    pub stale_check: StaleTracker,
}

impl UdpReceiveBlock {
    pub fn new(name: &'static str, stale_age_ms: f64) -> Self {
        UdpReceiveBlock {
            name,
            data: BlockData::from_bytes(b""),
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }

    pub fn run(&mut self, protocol: &mut dyn UdpProtocol, app_time_s: f64) {
        let mut buf = [0u8; BUFF_SIZE_BYTES];
        if let Ok(size) = protocol.read(&mut buf) {
            self.data.set_bytes(&buf[..size]);
            self.stale_check.mark_updated(app_time_s);
            debug!("{}: Received data: {:?}", self.name, &buf[..size])
        }
    }
}

impl IsValid for UdpReceiveBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocols::MockUdpProtocol;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_reads_data_from_socket() {
        let mut mock_proto = MockUdpProtocol::new();
        let res = "1234";
        mock_proto.expect_read().return_once(move |buf| {
            for (i, c) in res.as_bytes().iter().enumerate() {
                buf[i] = *c;
            }

            Ok(res.len())
        });

        let mut block = UdpReceiveBlock::new("Foo", 1000.0);
        block.run(&mut mock_proto, 0.1);
        assert_eq!(block.data, BlockData::from_bytes(res.as_bytes()));
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_fails_to_read_data_from_socket() {
        let mut mock_proto = MockUdpProtocol::new();
        mock_proto
            .expect_read()
            .return_once(|_| Err(Error::new(ErrorKind::Other, "oh no!")));

        let mut block = UdpReceiveBlock::new("Foo", 1000.0);
        block.run(&mut mock_proto, 0.1);
        assert_eq!(block.data, BlockData::from_bytes(b""));
        assert!(!block.is_valid(0.1).all());
    }
}
