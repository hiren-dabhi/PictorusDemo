use alloc::string::String;

use crate::block_data::{BlockData, BlockDataType};
use embedded_hal::digital::OutputPin;
use log::debug;

// Control a GPIO pin
pub struct GpioOutputBlock {
    name: String,
}

impl GpioOutputBlock {
    pub fn new(name: &str) -> GpioOutputBlock {
        GpioOutputBlock {
            name: String::from(name),
        }
    }
    pub fn run(&mut self, input: &BlockData, proto: &mut impl OutputPin) {
        match input.get_type() {
            BlockDataType::Scalar => {
                debug!("{}: {:?}", self.name, input);
                let high = input.any();
                debug!(
                    "{} switching pin {}",
                    self.name,
                    if high { "on" } else { "off" }
                );

                if high {
                    proto.set_high().ok();
                } else {
                    proto.set_low().ok();
                }
            }
            _ => panic!("Not Implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocols::MockOutputPin;

    #[test]
    fn test_gpio_output_block_sets_pin_high() {
        let mut proto = MockOutputPin::new();
        proto.expect_set_high().return_const(Ok(()));

        let mut block = GpioOutputBlock::new("GpioOutput1");
        block.run(&BlockData::from_scalar(1.0), &mut proto);
    }

    #[test]
    fn test_gpio_output_block_sets_pin_low() {
        let mut proto = MockOutputPin::new();
        proto.expect_set_low().return_const(Ok(()));

        let mut block = GpioOutputBlock::new("GpioOutput1");
        block.run(&BlockData::from_scalar(0.0), &mut proto);
    }
}
