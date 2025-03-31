use crate::{InputBlock, OutputBlock};

pub type GpioOutputBlock<'m> = &'m dyn OutputBlock<Inputs = bool>;
pub type SerialReceiveBlock<'m> = &'m dyn InputBlock<Output = [u8]>;
pub type SerialTransmitBlock<'m> = &'m dyn OutputBlock<Inputs = [u8]>;
