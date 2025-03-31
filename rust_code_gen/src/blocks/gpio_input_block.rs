use alloc::string::String;

use crate::block_data::{BlockData, BlockDataType};
use embedded_hal::digital::InputPin;
use log::debug;

// Read GPIO pin
pub struct GpioInputBlock {
    pub name: String,
    pub data: BlockData,
}
impl GpioInputBlock {
    pub fn new(name: &str) -> GpioInputBlock {
        GpioInputBlock {
            name: String::from(name),
            data: BlockData::from_scalar(0.0),
        }
    }
    pub fn run(&mut self, proto: &mut impl InputPin) {
        let pin_on = proto.is_high().unwrap_or(false);
        match self.data.get_type() {
            BlockDataType::Scalar => self.data.set_scalar_bool(pin_on),
            _ => panic!("Not Implemented"),
        }
        debug!("{}: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use protocols::MockInputPin;

    #[test]
    fn test_gpio_input_block_reads_pin_high() {
        let mut proto = MockInputPin::new();
        proto.expect_is_high().return_const(Ok(true));

        let mut block = GpioInputBlock::new("Gpio1");
        block.run(&mut proto);
        assert_eq!(block.data.scalar(), 1.0)
    }

    #[test]
    fn test_gpio_input_block_reads_pin_low() {
        let mut proto = MockInputPin::new();
        proto.expect_is_high().return_const(Ok(false));

        let mut block = GpioInputBlock::new("Gpio1");
        block.run(&mut proto);
        assert_eq!(block.data.scalar(), 0.0)
    }
}
