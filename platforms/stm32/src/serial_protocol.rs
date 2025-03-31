use alloc::vec::Vec;
use embassy_futures::{
    block_on,
    select::{select, Either},
};
use embassy_stm32::usart::Error;
#[cfg(feature = "interrupt-uart")]
use embassy_stm32::usart::{BufferedUart, BufferedUartRx, BufferedUartTx};
#[cfg(not(feature = "interrupt-uart"))]
use embassy_stm32::{
    mode::Async,
    usart::{RingBufferedUartRx, Uart, UartTx},
};
use embassy_time::{Duration, Timer};
use embedded_io::{ErrorType, Read, Write};
use embedded_io_async as a_io;

pub struct SerialWrapper<'a> {
    #[cfg(not(feature = "interrupt-uart"))]
    tx: UartTx<'a, Async>,
    #[cfg(feature = "interrupt-uart")]
    tx: BufferedUartTx<'a>,
    #[cfg(not(feature = "interrupt-uart"))]
    rx: RingBufferedUartRx<'a>,
    #[cfg(feature = "interrupt-uart")]
    rx: BufferedUartRx<'a>,
    // True if the cached data is invalid and should not be read from
    // False if the cache is good to read from
    cache_stale: bool,
    cache: Vec<u8>,
}

const CAPACITY: usize = 1024;
impl<'a> SerialWrapper<'a> {
    #[cfg(not(feature = "interrupt-uart"))]
    pub fn new(uart: Uart<'a, Async>, rx_buf: &'a mut [u8]) -> Self {
        let (tx, rx) = uart.split();
        let mut rx = rx.into_ring_buffered(rx_buf);
        rx.start().unwrap();
        Self {
            tx,
            rx,
            cache_stale: true,
            cache: Vec::with_capacity(CAPACITY),
        }
    }

    #[cfg(feature = "interrupt-uart")]
    pub fn new(uart: BufferedUart<'a>) -> Self {
        let (tx, rx) = uart.split();
        Self {
            tx,
            rx,
            cache_stale: true,
            cache: Vec::with_capacity(CAPACITY),
        }
    }
}

impl ErrorType for SerialWrapper<'_> {
    type Error = Error;
}

impl Read for SerialWrapper<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if self.cache_stale {
            // Regardless of the result we don't want to read again until flush is called
            self.cache_stale = false;

            self.cache.resize(CAPACITY, 0);
            let read_fut = a_io::Read::read(&mut self.rx, &mut self.cache);
            let time_fut = Timer::after(Duration::from_micros(10));

            // Wait for either the read to finish or a short timer to expire
            match block_on(select(read_fut, time_fut)) {
                // The timer went off, which means no data was read
                Either::Second(_) => self.cache.clear(),
                Either::First(Err(e)) => {
                    self.cache.clear();
                    return Err(e);
                }
                // Shrink the cache to only include the data we read
                Either::First(Ok(size)) => self.cache.resize(size, 0),
            }
        }

        // Return cached data that we possibly read during this call
        let len = buf.len().min(self.cache.len());
        if len == 0 {
            // Not sure what the correct error is here
            return Err(Error::Framing);
        }

        buf[..len].copy_from_slice(&self.cache[..len]);
        Ok(len)
    }
}

impl Write for SerialWrapper<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Write::write(&mut self.tx, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.cache_stale = true;
        self.cache.clear();
        Ok(())
    }
}
