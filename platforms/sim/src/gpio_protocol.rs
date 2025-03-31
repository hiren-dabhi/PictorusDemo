use std::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

pub struct SimGpioPin {}

impl ErrorType for SimGpioPin {
    type Error = Infallible;
}

impl InputPin for SimGpioPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl OutputPin for SimGpioPin {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn create_gpio_input_pin(_: f64) -> Result<SimGpioPin, Infallible> {
    Ok(SimGpioPin {})
}

pub fn create_gpio_output_pin(_: f64) -> Result<SimGpioPin, Infallible> {
    Ok(SimGpioPin {})
}
