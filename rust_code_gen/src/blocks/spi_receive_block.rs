use crate::{block_data::BlockData, stale_tracker::StaleTracker, traits::IsValid};
use log::debug;
use protocols::SpiProtocol;

const SPI_RECEIVE_BUFFER_SIZE: usize = 256;

pub struct SpiReceiveBlock {
    name: &'static str,
    buffer: [u8; SPI_RECEIVE_BUFFER_SIZE],
    bytes_to_read: usize,
    pub stale_check: StaleTracker,
    pub data: BlockData,
}

impl SpiReceiveBlock {
    pub fn new(name: &'static str, stale_age_ms: f64, bytes_to_read: f64) -> Self {
        SpiReceiveBlock {
            name,
            buffer: [0; SPI_RECEIVE_BUFFER_SIZE],
            stale_check: StaleTracker::from_ms(stale_age_ms),
            bytes_to_read: bytes_to_read as usize,
            data: BlockData::from_bytes(b""),
        }
    }

    pub fn run(&mut self, protocol: &mut impl SpiProtocol, app_time_s: f64) {
        let mut bytes_to_read_actual = self.bytes_to_read;
        if self.bytes_to_read > SPI_RECEIVE_BUFFER_SIZE {
            bytes_to_read_actual = SPI_RECEIVE_BUFFER_SIZE;
            debug!(
                "{}: Attempted to read more bytes than buffer size, reading {} bytes",
                self.name, SPI_RECEIVE_BUFFER_SIZE
            );
        }

        match protocol.read(&mut self.buffer[..bytes_to_read_actual]) {
            Ok(()) => {
                let val = &self.buffer[..bytes_to_read_actual];
                self.stale_check.mark_updated(app_time_s);
                self.data.set_bytes(val);
                debug!("{}: Read {} bytes", self.name, bytes_to_read_actual);
            }
            Err(_) => {
                debug!("{}: Failed to read SPI data", self.name);
            }
        }
    }
}

impl IsValid for SpiReceiveBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use embedded_hal::spi::ErrorKind;
    use protocols::MockSpiProtocol;

    #[test]
    fn test_spi_receive_block_read_bytes() {
        // Mocking MB85RS4 Status register response
        let response = [0x04, 0x7f, 0x49, 0x03];

        let mut mock_proto = MockSpiProtocol::new();
        mock_proto.expect_read().times(1).return_once(move |buf| {
            for (i, byte) in response.iter().enumerate() {
                buf[i] = *byte;
            }
            Ok(())
        });

        let mut block = SpiReceiveBlock::new("Foo", 1000.0, 4.0);
        block.run(&mut mock_proto, 0.0);

        assert_eq!(block.data, BlockData::from_bytes(&response));
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_spi_receive_block_fails_to_read_bytes() {
        let mut mock_proto = MockSpiProtocol::new();
        mock_proto
            .expect_read()
            .times(1)
            .return_once(|_| Err(ErrorKind::Overrun));

        let mut block = SpiReceiveBlock::new("Foo", 1000.0, 4.0);
        block.run(&mut mock_proto, 0.0);

        assert_eq!(block.data, BlockData::from_bytes(b""));
        assert!(!block.is_valid(0.0).all());
    }
}
