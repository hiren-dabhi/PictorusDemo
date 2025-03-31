use embedded_hal::i2c::I2c;
use log::debug;

use crate::block_data::BlockData;

pub struct I2cOutputBlock {
    name: &'static str,
    addr: u8,
    cmd: u8,
}
impl I2cOutputBlock {
    pub fn new(name: &'static str, addr: f64, command: f64) -> Self {
        Self {
            name,
            addr: addr as u8,
            cmd: command as u8,
        }
    }

    pub fn run(&self, input: &BlockData, protocol: &mut impl I2c) {
        let register = &[self.cmd];
        let data = input.to_bytes();
        let reg_data = [register, data.as_slice()].concat();
        debug!("{}: Transmitting value: {:?}", self.name, &reg_data);
        protocol.write(self.addr, &reg_data).ok();
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use protocols::MockI2cProtocol;

    #[test]
    fn test_i2c_output_writes_simple_data() {
        let input = 255f64;
        let address = 99.0;
        let command = 10u8;

        let mut proto = MockI2cProtocol::new();

        proto
            .expect_write()
            .withf(move |&addr, buf| {
                addr == address as u8 && buf[0] == command && buf[1..] == input.to_be_bytes()
            })
            .return_once(|_, _| Ok(()));

        let block = I2cOutputBlock::new("I2cOutput1", address, command as f64);
        block.run(&BlockData::from_bytes(&input.to_be_bytes()), &mut proto);
    }
}
