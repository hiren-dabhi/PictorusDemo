use core::convert::Infallible;

pub use embedded_hal::i2c::I2c;
use embedded_hal::i2c::{ErrorType, Operation};

pub struct SimI2cProtocol {}
pub type I2cProtocolType = SimI2cProtocol;

impl ErrorType for SimI2cProtocol {
    type Error = Infallible;
}

impl I2c for SimI2cProtocol {
    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn create_i2c_protocol() -> Result<SimI2cProtocol, Infallible> {
    Ok(SimI2cProtocol {})
}
