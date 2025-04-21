use core::convert::Infallible;
use corelib_traits::{ByteSliceSignal, InputBlock, OutputBlock};
use embedded_io::{ErrorType, Read, Write};
use pictorus_core_blocks::{SerialReceiveBlockParams, SerialTransmitBlockParams};

pub struct SerialConnection {
    buffer: alloc::vec::Vec<u8>,
}

impl SerialConnection {
    pub fn new(_port: &str, _baud: f64, _transmit_enabled: bool) -> Result<Self, Infallible> {
        Ok(SerialConnection {
            buffer: alloc::vec::Vec::new(),
        })
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

impl OutputBlock for SerialConnection {
    type Inputs = corelib_traits::ByteSliceSignal;
    type Parameters = SerialTransmitBlockParams;

    fn output(
        &mut self,
        _parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: corelib_traits::PassBy<'_, Self::Inputs>,
    ) {
        self.write(inputs).ok();
    }
}

impl InputBlock for SerialConnection {
    type Output = ByteSliceSignal;
    type Parameters = SerialReceiveBlockParams;

    fn input(
        &mut self,
        _parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
    ) -> corelib_traits::PassBy<'_, Self::Output> {
        &self.buffer
    }
}
