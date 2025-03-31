use std::convert::Infallible;

use protocols::SpiProtocol;

pub struct SimSpi {}

impl SimSpi {
    pub fn new() -> Result<Self, Infallible> {
        Ok(SimSpi {})
    }
}

impl SpiProtocol for SimSpi {
    type Error = Infallible;

    fn write(&mut self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read(&mut self, _data: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}
