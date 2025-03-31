use std::convert::Infallible;

use embedded_can::nb::Can;
use protocols::CanProtocol;

pub struct SimFrame {}
impl embedded_can::Frame for SimFrame {
    fn new(_id: impl Into<embedded_can::Id>, _data: &[u8]) -> Option<Self> {
        Some(Self {})
    }

    fn new_remote(_id: impl Into<embedded_can::Id>, _dlc: usize) -> Option<Self> {
        Some(Self {})
    }

    fn is_extended(&self) -> bool {
        false
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> embedded_can::Id {
        embedded_can::Id::Standard(embedded_can::StandardId::ZERO)
    }

    fn dlc(&self) -> usize {
        0
    }

    fn data(&self) -> &[u8] {
        &[]
    }
}

pub struct SimCan {
    frames: Vec<SimFrame>,
}

impl SimCan {
    pub fn new(_iface: &str) -> Result<Self, Infallible> {
        Ok(Self { frames: vec![] })
    }
}

impl Can for SimCan {
    type Frame = SimFrame;
    type Error = embedded_can::ErrorKind;

    fn transmit(&mut self, _frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        Ok(None)
    }
    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        Ok(SimFrame {})
    }
}

impl CanProtocol for SimCan {
    fn read_frames(&mut self) -> &[impl protocols::Frame] {
        &self.frames
    }

    fn flush(&mut self) {
        self.frames.clear();
    }
}
