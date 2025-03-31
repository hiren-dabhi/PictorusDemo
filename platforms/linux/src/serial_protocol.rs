use embedded_io::{ErrorType, Read, Write};
use log::{debug, info};
use serialport::{self, SerialPort};
use std::io;
use utils::PictorusError;

pub fn create_serial_port(
    port: &str,
    baud_rate: f64,
    transmit_enabled: bool,
) -> Result<Option<Box<dyn SerialPort>>, PictorusError> {
    if !transmit_enabled {
        return Ok(None);
    }

    let baud_rate = baud_rate as u32;
    let port = serialport::new(port, baud_rate).open().map_err(|err| {
                let message = match err.kind() {
                    serialport::ErrorKind::NoDevice => {
                        format!("Failed to bind to serial port: {} - This could indicate that the device is in use by another process or was disconnected while performing I/O.", port)
                    }
                    serialport::ErrorKind::Io(_) => {
                        format!("Failed to bind to serial port: {} - Does it exist?", port)
                    }
                    _ => format!(
                        "Unknown error! Unable to connect to serial port: {} ({})",
                        port,
                        err
                    ),
                };
                PictorusError::new("SerialProtocol".into(), message)
            })?;
    Ok(Some(port))
}

pub struct SerialConnection {
    port: Option<Box<dyn SerialPort>>,
    cache: Option<Vec<u8>>,
    port_addr: String,
}

impl SerialConnection {
    pub fn new(port: &str, baud: f64, transmit_enabled: bool) -> Result<Self, PictorusError> {
        info!("Opening serial port {} with baud {}", port, baud);
        Ok(SerialConnection {
            port: create_serial_port(port, baud, transmit_enabled)?,
            cache: None,
            port_addr: port.to_string(),
        })
    }
}

impl ErrorType for SerialConnection {
    type Error = io::Error;
}

impl Read for SerialConnection {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if self.cache.is_none() {
            if let Some(port) = &mut self.port {
                // Set the cache regardless of the result so we don't read again until flush is called
                let res = port.read(buf);
                let size = match res {
                    Ok(size) => size,
                    Err(err) => {
                        self.cache = Some(Vec::new());
                        return Err(err);
                    }
                };
                self.cache = Some(Vec::from(&buf[..size]));
            }
        }

        if let Some(cache) = &self.cache {
            if cache.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "No data available",
                ));
            }

            let len = buf.len().min(cache.len());
            buf[..len].copy_from_slice(&cache[..len]);
            return Ok(len);
        }
        Err(io::Error::new(io::ErrorKind::NotConnected, "I/O disabled"))
    }
}

impl Write for SerialConnection {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if let Some(port) = &mut self.port {
            return port.write(buf);
        }
        Err(io::Error::new(io::ErrorKind::NotConnected, "I/O disabled"))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.cache = None;
        Ok(())
    }
}

impl Drop for SerialConnection {
    fn drop(&mut self) {
        debug!("Closing serial port {}", self.port_addr);
        if let Some(port) = &mut self.port {
            let _ = port.flush();
        }
    }
}
