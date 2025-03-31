use log::debug;
use protocols::UdpProtocol;
use std::io::{Error, ErrorKind};
use std::net::UdpSocket;

use utils::PictorusError;

const ERR_TYPE: &str = "UdpProtocol";

fn create_udp_socket(
    address: &str,
    transmit_enabled: bool,
) -> Result<Option<UdpSocket>, PictorusError> {
    if !transmit_enabled {
        return Ok(None);
    }

    let socket = UdpSocket::bind(address).map_err(|err| {
        let message = match err.kind() {
            ErrorKind::InvalidInput => format!("Couldn't bind UDP receiver at invalid address: {} - Is this address valid?", address),
            ErrorKind::AddrInUse => format!("Couldn't bind UDP receiver at already bound address: {} - Is another process currently bound here?", address),
            _ => format!("Unknown error! Couldn't bind UDP receiver at address: {} ({})", address, err),
        };
        PictorusError::new(
            ERR_TYPE.into(),
            message,
        )
    })?;

    socket.set_nonblocking(true).map_err(|_| {
        PictorusError::new(
            ERR_TYPE.into(),
            format!(
                "Failed to set nonblocking on UDP port at address: {}",
                address
            ),
        )
    })?;
    Ok(Some(socket))
}

pub struct UdpConnection {
    socket: Option<UdpSocket>,
    cache: Option<Vec<u8>>,
}

impl UdpConnection {
    pub fn new(address: &str, transmit_enabled: bool) -> Result<Self, PictorusError> {
        Ok(UdpConnection {
            cache: None,
            socket: create_udp_socket(address, transmit_enabled)?,
        })
    }
}

// TODO: This is almost identical to SerialProtocol. We might be able to combine into a common trait
impl UdpProtocol for UdpConnection {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.cache.is_none() {
            if let Some(socket) = &mut self.socket {
                // Set the cache regardless of the result so we don't read again until flush is called
                let mut num_bytes_read = 0;
                // Only use the most recent value on the socket
                while let Ok((n, _)) = socket.recv_from(buf) {
                    num_bytes_read = n;
                }

                debug!("Received {} bytes", num_bytes_read);
                self.cache = Some(Vec::from(&buf[..num_bytes_read]));
            }
        }

        if let Some(cache) = &self.cache {
            if cache.is_empty() {
                return Err(Error::new(ErrorKind::WouldBlock, "No data received"));
            }

            let len = buf.len().min(cache.len());
            buf[..len].copy_from_slice(&cache[..len]);
            return Ok(len);
        }

        Err(Error::new(ErrorKind::NotConnected, "I/O disabled"))
    }

    fn write(&mut self, buf: &[u8], to_addr: &str) -> Result<usize, Error> {
        if let Some(socket) = &mut self.socket {
            return socket.send_to(buf, to_addr);
        }

        Ok(0)
    }

    fn flush(&mut self) {
        self.cache = None;
    }
}
