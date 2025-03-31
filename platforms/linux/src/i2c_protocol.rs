pub use embedded_hal_02::blocking::i2c::{Write, WriteRead};

use utils::PictorusError;

use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
pub use linux_embedded_hal::I2cdev;

const ERR_TYPE: &str = "I2cProtocol";
// TODO: This should be configurable by block param
const I2C_PATH: &str = "/dev/i2c-1";

pub fn create_i2c_protocol() -> Result<I2cdev, PictorusError> {
    let i2c = I2cdev::new(I2C_PATH).map_err(|err| {
        let msg = match err {
            LinuxI2CError::Errno(e) => format!(
                "Unknown error! Failed to bind to I2C device: {} ({})",
                I2C_PATH, e
            ),
            LinuxI2CError::Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => format!(
                    "Failed to bind to I2C device: {} - not found. Is the I2C bus enabled?",
                    I2C_PATH
                ),
                _ => format!(
                    "Unknown error! Failed to bind to I2C device: {} ({})",
                    I2C_PATH, e
                ),
            },
        };
        PictorusError::new(ERR_TYPE.into(), msg)
    })?;

    Ok(i2c)
}
