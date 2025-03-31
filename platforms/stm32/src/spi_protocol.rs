use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Blocking;
use embassy_stm32::spi::Spi;
use log::warn;
use protocols::SpiProtocol;

pub struct SpiWrapper<'a> {
    spi: Spi<'a, Blocking>,
    bits_per_transfer: u8,
    cs: Output<'a>,
}

impl<'a> SpiWrapper<'a> {
    pub fn new(spi: Spi<'a, Blocking>, bits_per_transfer: u8, cs_pin: Output<'a>) -> Self {
        Self {
            spi,
            bits_per_transfer,
            cs: cs_pin,
        }
    }
}

impl SpiProtocol for SpiWrapper<'_> {
    type Error = embassy_stm32::spi::Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low();
        match self.bits_per_transfer {
            1..=8 => self.spi.blocking_write(data),
            9..=16 => {
                if data.len() % 2 != 0 {
                    warn!("Data length is not a multiple of 2, dropping last byte");
                }

                let result = data.chunks_exact(2).try_for_each(|chunk| {
                    let mut val = [0u16; 1];
                    val[0] = u16::from_le_bytes([chunk[1], chunk[0]]);
                    self.spi.blocking_write(&[val[0]])
                });
                result
            }
            _ => self.spi.blocking_write(data),
        }
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        let retval = self.spi.blocking_read(data);
        self.cs.set_high();
        retval
    }
}
