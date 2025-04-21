#[cfg(test)]
pub(crate) mod constant_block;

#[cfg(test)]
pub(crate) mod gain_block;

#[cfg(test)]
pub(crate) mod product_block;

#[cfg(test)]
mod sinewave_block;

#[cfg(test)]
pub(crate) mod sum_block;

mod pwm_block;
pub use pwm_block::*;

#[cfg(any(feature = "can", feature = "fdcan"))]
mod can_receive_block;
#[cfg(any(feature = "can", feature = "fdcan"))]
pub use can_receive_block::*;

#[cfg(any(feature = "can", feature = "fdcan"))]
mod can_transmit_block;
#[cfg(any(feature = "can", feature = "fdcan"))]
pub use can_transmit_block::*;

// Declare all block modules and re-export them to make them importable from crate::blocks
cfg_if::cfg_if! {
  if #[cfg(feature = "std")] {
    #[cfg(target_arch = "x86_64")]
    mod fmu_block;
    #[cfg(target_arch = "x86_64")]
    pub use fmu_block::*;
  }
}
