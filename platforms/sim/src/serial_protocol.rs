use core::convert::Infallible;
use embedded_io::{ErrorType, Read, Write};

pub struct SerialConnection {}

impl SerialConnection {
    pub fn new(_port: &str, _baud: f64, _transmit_enabled: bool) -> Result<Self, Infallible> {
        Ok(SerialConnection {})
    }
}

impl ErrorType for SerialConnection {
    type Error = Infallible;
}

impl Read for SerialConnection {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(0)
    }
}

impl Write for SerialConnection {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
