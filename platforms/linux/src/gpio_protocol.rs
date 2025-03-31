pub use embedded_hal::digital::{InputPin, OutputPin};
use utils::PictorusError;

use alloc::{format, string::String};

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
pub use linux_embedded_hal::CdevPin;

// TODO: This should be configurable by block param
const GPIO_CHIP: &str = "/dev/gpiochip0";
const ERR_TYPE: &str = "GpioProtocol";

fn create_error(message: String) -> PictorusError {
    PictorusError::new(ERR_TYPE.into(), message)
}

fn create_pin_error(pin: u32) -> PictorusError {
    create_error(format!("Failed to bind to GPIO pin: {}", pin))
}

fn create_cdev_pin(
    chip: &str,
    pin_line: f64,
    flag: LineRequestFlags,
) -> Result<CdevPin, PictorusError> {
    let pin_line = pin_line as u32;
    let mut chip = Chip::new(chip).map_err(|_| {
        create_error(format!(
            "Failed to bind to GPIO bus {} for pin: {}",
            chip, pin_line
        ))
    })?;
    let handle = chip
        .get_line(pin_line)
        // TODO: Might be cleaner to impl From<linux_embedded_hal::gpio_cdev::Error> for PictorusError
        .map_err(|_| create_pin_error(pin_line))?
        .request(flag, 0, "pictorus")
        .map_err(|_| create_pin_error(pin_line))?;

    CdevPin::new(handle).map_err(|_| create_pin_error(pin_line))
}

pub fn create_gpio_input_pin(pin_number: f64) -> Result<CdevPin, PictorusError> {
    create_cdev_pin(GPIO_CHIP, pin_number, LineRequestFlags::INPUT)
}

pub fn create_gpio_output_pin(pin_number: f64) -> Result<CdevPin, PictorusError> {
    create_cdev_pin(GPIO_CHIP, pin_number, LineRequestFlags::OUTPUT)
}
