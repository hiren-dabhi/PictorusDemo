extern crate alloc;

mod camera_protocol;
pub use camera_protocol::*;

mod clock_protocol;
pub use clock_protocol::*;

mod delay_protocol;
pub use delay_protocol::*;

// pub mod display_protocol;

mod gpio_protocol;
pub use gpio_protocol::*;

mod i2c_protocol;
pub use i2c_protocol::*;

mod pwm_protocol;
pub use pwm_protocol::*;

mod serial_protocol;
pub use serial_protocol::*;

mod udp_protocol;
pub use udp_protocol::*;

mod can_protocol;
pub use can_protocol::*;

#[cfg(feature = "spi")]
mod spi_protocol;
#[cfg(feature = "spi")]
pub use spi_protocol::*;
