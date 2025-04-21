#![no_std]

// gpio protocols
pub use embedded_hal::digital::{InputPin, OutputPin};

// i2c protocol
pub use embedded_hal::i2c::I2c;

// pwm protocol
pub use embedded_hal_02::Pwm;

// serial protocol
pub use embedded_io::{Read, Write};

// clock protocol
pub use embedded_time::Clock;

// CAN protocol
#[cfg(any(feature = "can", feature = "fdcan"))]
pub use embedded_can::{nb::Can, Frame};

#[cfg(feature = "std")]
extern crate std;

// TODO: These trait wrappers are kind of dumb. But to move away from them we need to handle
// buffering in the main file rather than in the protocol implementations.
#[cfg(any(feature = "can", feature = "fdcan"))]
pub trait CanProtocol: Can {
    fn read_frames(&mut self) -> &[impl Frame];

    fn flush(&mut self);
}

#[cfg(feature = "std")]
pub trait UdpProtocol {
    fn read(&mut self) -> Result<&[u8], std::io::Error>;
    fn write(&mut self, buf: &[u8], to_addr: &str) -> Result<usize, std::io::Error>;
    fn flush(&mut self);
}

pub trait Flush {
    fn flush(&mut self);
}

#[cfg(feature = "adc")]
pub trait AdcProtocol {
    fn read(&mut self) -> u16;
    fn flush(&mut self);
}

#[cfg(feature = "dac")]
pub trait DacProtocol<const CHANNELS: usize, const SAMPLES: usize> {
    /// Trait function to write a buffer of samples to the DAC. The samples are the ROWS and the
    /// channels are the COLUMNS. For example, 2 channels and 1 sample for each channel
    /// would require a reference to a buffer of data that is sized &[[u16; 1]; 2].
    fn write(&mut self, value: &[[u16; SAMPLES]; CHANNELS]);
}

#[cfg(all(feature = "test-utils", feature = "std"))]
pub use test_utils::*;

#[cfg(all(feature = "test-utils", feature = "std"))]
mod test_utils {
    use super::*;
    extern crate std;
    use std::prelude::rust_2021::*;

    pub use embedded_hal::delay::DelayNs;
    use embedded_hal::digital;
    use embedded_hal::i2c::{self, Operation};
    use embedded_io as io;
    use embedded_time::rate::Fraction;
    use mockall::mock;

    mock! {
        Clock {}
        impl Clock for Clock {
            type T = u64;

            const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

            fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
                todo!()
            }
        }
    }

    mock! {
        DelayNs {}
        impl DelayNs for DelayNs {
            fn delay_ns(&mut self, ns: u32);
        }
    }

    mock! {
        pub InputPin {}
        impl digital::ErrorType for InputPin {
            type Error = digital::ErrorKind;
        }
        impl InputPin for InputPin {
            fn is_high(&mut self) -> Result<bool, digital::ErrorKind>;

            fn is_low(&mut self) -> Result<bool, digital::ErrorKind>;
        }
    }

    mock! {
        pub OutputPin {}
        impl digital::ErrorType for OutputPin {
            type Error = digital::ErrorKind;
        }
        impl OutputPin for OutputPin {
            fn set_low(&mut self) -> Result<(), digital::ErrorKind>;

            fn set_high(&mut self) -> Result<(), digital::ErrorKind>;
        }
    }

    mock! {
        pub I2cProtocol {}
        impl i2c::ErrorType for I2cProtocol {
            type Error = i2c::ErrorKind;
        }
        impl I2c for I2cProtocol {
            fn transaction<'a>(
                &mut self,
                address: u8,
                operations: &mut [Operation<'a>],
            ) -> Result<(), i2c::ErrorKind>;

            fn write_read(
                &mut self,
                address: u8,
                write: &[u8],
                read: &mut [u8]
            ) -> Result<(), i2c::ErrorKind>;

            fn write(
                &mut self,
                address: u8,
                write: &[u8]
            ) -> Result<(), i2c::ErrorKind>;
        }
    }

    mock! {
        pub Write {}
        impl io::ErrorType for Write {
            type Error = io::ErrorKind;
        }
        impl io::Write for Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, io::ErrorKind>;
            fn flush(&mut self) -> Result<(), io::ErrorKind>;
        }
    }

    mock! {
        pub Read {}
        impl io::ErrorType for Read {
            type Error = io::ErrorKind;
        }
        impl io::Read for Read {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::ErrorKind>;
        }
    }

    pub struct MockFrame {
        id: embedded_can::Id,
        data: Vec<u8>,
    }
    impl embedded_can::Frame for MockFrame {
        fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
            Some(Self {
                id: id.into(),
                data: data.to_vec(),
            })
        }

        fn new_remote(id: impl Into<embedded_can::Id>, _dlc: usize) -> Option<Self> {
            Self::new(id, &[])
        }

        fn is_extended(&self) -> bool {
            false
        }

        fn is_remote_frame(&self) -> bool {
            false
        }

        fn id(&self) -> embedded_can::Id {
            self.id
        }

        fn dlc(&self) -> usize {
            0
        }

        fn data(&self) -> &[u8] {
            &self.data
        }
    }

    #[cfg(any(feature = "can", feature = "fdcan"))]
    mock! {
        pub CanProtocol {}
        impl Can for CanProtocol {
            type Frame = MockFrame;
            type Error = embedded_can::ErrorKind;

            fn transmit(&mut self, frame: &MockFrame) -> Result<Option<MockFrame>, nb::Error<embedded_can::ErrorKind>>;
            fn receive(&mut self) -> Result<MockFrame, nb::Error<embedded_can::ErrorKind>>;
        }

        impl CanProtocol for CanProtocol {
            #[allow(refining_impl_trait)]
            fn read_frames(&mut self) -> &[MockFrame];
            fn flush(&mut self);
        }
    }

    mock! {
        pub FlushableProtocol {}

        impl Flush for FlushableProtocol {
            fn flush(&mut self);
        }
    }

    #[cfg(feature = "adc")]
    mock! {
        pub AdcProtocol {}
        impl AdcProtocol for AdcProtocol {
            fn read(&mut self) -> u16;
            fn flush(&mut self);
        }
    }

    #[cfg(feature = "dac")]
    mock! {
        // DacProtocol requires the number of channels (2) and the number of samples (1)
        pub DacProtocol {}
        impl DacProtocol<2, 1> for DacProtocol {
            fn write(&mut self, value: &[[u16; 1]; 2]);
        }
    }
}
