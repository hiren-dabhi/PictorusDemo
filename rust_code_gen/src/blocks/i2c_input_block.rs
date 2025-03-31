use alloc::vec;
use core::cmp::min;

use embedded_hal::i2c::I2c;

use crate::{
    block_data::BlockData, byte_data::BUFF_SIZE_BYTES, stale_tracker::StaleTracker, traits::IsValid,
};
use log::debug;

pub struct I2cInputBlock {
    name: &'static str,
    cmd: u8,
    addr: u8,
    buff_size: usize,
    pub data: BlockData,
    pub stale_check: StaleTracker,
}
impl I2cInputBlock {
    pub fn new(
        name: &'static str,
        addr: f64,
        command: f64,
        buff_size: f64,
        stale_age_ms: f64,
    ) -> Self {
        Self {
            name,
            data: BlockData::from_bytes(b""),
            addr: addr as u8,
            cmd: command as u8,
            buff_size: min(buff_size as usize, BUFF_SIZE_BYTES),
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }

    pub fn run(&mut self, protocol: &mut impl I2c, app_time_s: f64) {
        let mut buff = vec![0u8; self.buff_size];
        let res = protocol.write_read(self.addr, &[self.cmd], buff.as_mut_slice());
        match res {
            Ok(()) => {
                self.data.set_bytes(&buff);
                self.stale_check.mark_updated(app_time_s);
            }
            Err(_) => {
                // TODO: Better error handling
                debug!("{}: Failed to read I2C data", self.name);
            }
        }
        debug!("{}: {:?}", self.name, self.data);
    }
}

impl IsValid for I2cInputBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, ByteOrder};
    use embedded_hal::i2c::ErrorKind;
    use protocols::MockI2cProtocol;

    #[test]
    fn test_i2c_input_block_reads_bytes() {
        let mut res_buf = [0; 8];
        BigEndian::write_f64(&mut res_buf, 1.234);

        let address = 1.0;
        let register = 42.0;
        let mut protocol = MockI2cProtocol::new();
        protocol
            .expect_write_read()
            .return_once(move |addr, bytes, buf| {
                assert_eq!(addr, address as u8);
                assert_eq!(bytes, &[register as u8]);
                for (i, c) in res_buf.iter().enumerate() {
                    buf[i] = *c;
                }
                Ok(())
            });

        let mut block = I2cInputBlock::new("I2cInput1", 1.0, register, 8.0, 1000.0);
        block.run(&mut protocol, 0.0);

        assert_eq!(block.data, BlockData::from_bytes(&res_buf));
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_i2c_input_block_fails_to_read_bytes() {
        let address = 1.0;
        let register = 42.0;
        let mut protocol = MockI2cProtocol::new();
        protocol
            .expect_write_read()
            .return_once(move |addr, bytes, _| {
                assert_eq!(addr, address as u8);
                assert_eq!(bytes, &[register as u8]);
                Err(ErrorKind::Other)
            });

        let mut block = I2cInputBlock::new("I2cInput1", address, register, 8.0, 1000.0);
        block.run(&mut protocol, 0.0);

        assert_eq!(block.data, BlockData::from_bytes(b""));
        assert!(!block.is_valid(0.0).all());
    }
}
