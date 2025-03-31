use protocols::UdpProtocol;
use std::{convert::Infallible, io::Error};

pub struct UdpConnection {}

impl UdpConnection {
    pub fn new(_addr: &str, _transmit_enabled: bool) -> Result<Self, Infallible> {
        Ok(UdpConnection {})
    }
}

impl UdpProtocol for UdpConnection {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    fn write(&mut self, buf: &[u8], _to_addr: &str) -> Result<usize, Error> {
        Ok(buf.len())
    }

    fn flush(&mut self) {}
}

pub fn create_udp_socket(_address: &str, _transmit_enabled: bool) -> UdpConnection {
    UdpConnection {}
}
