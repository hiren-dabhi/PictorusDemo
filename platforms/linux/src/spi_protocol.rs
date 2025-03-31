use std::io::{Read, Write};

use linux_embedded_hal::spidev::{Spidev, SpidevOptions};
use linux_embedded_hal::CdevPin;
use protocols::SpiProtocol;
use utils::PictorusError;

pub struct SpiConnection {
    device: Spidev,
    cs: CdevPin,
}

impl SpiConnection {
    pub fn new(
        port: &'static str,
        frequency: u32,
        bits_per_transfer: u8,
        lsb_first: bool,
        mode: &'static str,
        cs: CdevPin,
    ) -> Result<Self, PictorusError> {
        let mut spi = Spidev::open(port).map_err(|_err| {
            PictorusError::new("SpiConnection".into(), "Failed to open SPI device".into())
        })?;

        let mut options = SpidevOptions::new();
        match mode {
            "1" => options.mode(linux_embedded_hal::spidev::SpiModeFlags::SPI_MODE_1),
            "2" => options.mode(linux_embedded_hal::spidev::SpiModeFlags::SPI_MODE_2),
            "3" => options.mode(linux_embedded_hal::spidev::SpiModeFlags::SPI_MODE_3),
            _ => options.mode(linux_embedded_hal::spidev::SpiModeFlags::SPI_MODE_0),
        };

        options
            .bits_per_word(bits_per_transfer)
            .max_speed_hz(frequency)
            .lsb_first(lsb_first);

        spi.configure(&options).map_err(|_err| {
            PictorusError::new(
                "SpiConnection".into(),
                "Failed to configure SPI device".into(),
            )
        })?;

        Ok(SpiConnection { device: spi, cs })
    }
}

impl SpiProtocol for SpiConnection {
    type Error = PictorusError;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_value(0).map_err(|_err| {
            PictorusError::new(
                "SpiConnection".into(),
                "Failed to set CS pin in ::write".into(),
            )
        })?;
        self.device.write(data).map_err(|_err| {
            PictorusError::new(
                "SpiConnection".into(),
                "Failed to write to SPI device in ::write_u8".into(),
            )
        })?;
        Ok(())
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        self.device.read_exact(data).map_err(|_err| {
            PictorusError::new(
                "SpiConnection".into(),
                "Failed to read from SPI device in ::read".into(),
            )
        })?;
        self.cs.set_value(1).map_err(|_err| {
            PictorusError::new(
                "SpiConnection".into(),
                "Failed to set CS pin in ::write".into(),
            )
        })?;
        Ok(())
    }
}
