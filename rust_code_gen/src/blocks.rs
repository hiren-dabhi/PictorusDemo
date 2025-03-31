mod bitwise_operator_block;
pub use bitwise_operator_block::*;

mod bytes_split_block;
pub use bytes_split_block::*;

mod bytes_join_block;
pub use bytes_join_block::*;

mod change_detection_block;
pub use change_detection_block::*;

mod component_input_block;
pub use component_input_block::*;

mod component_output_block;
pub use component_output_block::*;

#[cfg(test)]
pub(crate) mod constant_block;

mod data_read_block;
pub use data_read_block::*;

mod data_write_block;
pub use data_write_block::*;

mod deadband_block;
pub use deadband_block::*;

mod delay_block;
pub use delay_block::*;

mod delay_control_block;
pub use delay_control_block::*;

mod derivative_block;
pub use derivative_block::*;

mod determinant_block;
pub use determinant_block::*;

mod dot_product_block;
pub use dot_product_block::*;

mod equation_block;
pub use equation_block::*;

mod exponent_block;
pub use exponent_block::*;

mod frequency_filter_block;
pub use frequency_filter_block::*;

#[cfg(test)]
pub(crate) mod gain_block;

mod gpio_input_block;
pub use gpio_input_block::*;

mod gpio_output_block;
pub use gpio_output_block::*;

mod histogram_plot_block;
pub use histogram_plot_block::*;

mod iir_filter_block;
pub use iir_filter_block::*;

mod inspect_block;
pub use inspect_block::*;

mod integral_block;
pub use integral_block::*;

mod json_dump_block;
pub use json_dump_block::*;

mod logical_block;
pub use logical_block::*;

mod lookup_1d_block;
pub use lookup_1d_block::*;

mod matrix_inverse_block;
pub use matrix_inverse_block::*;

mod min_max_block;
pub use min_max_block::*;

mod not_block;
pub use not_block::*;

mod pid_block;
pub use pid_block::*;

mod plot_block;
pub use plot_block::*;

mod product_block;
pub use product_block::*;

mod quantize_block;
pub use quantize_block::*;

mod rate_limit_block;
pub use rate_limit_block::*;

mod rust_code_block;
pub use rust_code_block::*;

mod scatter_plot_block;
pub use scatter_plot_block::*;

#[cfg(test)]
mod sinewave_block;

mod sliding_window_block;
pub use sliding_window_block::*;

mod state_transition_block;
pub use state_transition_block::*;

mod string_format_block;
pub use string_format_block::*;

#[cfg(test)]
pub(crate) mod sum_block;

mod switch_block;
pub use switch_block::*;

mod timer_block;
pub use timer_block::*;

mod transfer_function_block;
pub use transfer_function_block::*;

mod transpose_block;
pub use transpose_block::*;

mod trigonometry_block;
pub use trigonometry_block::*;

mod vector_merge_block;
pub use vector_merge_block::*;

mod vector_norm_block;
pub use vector_norm_block::*;

mod vector_reshape_block;
pub use vector_reshape_block::*;

mod vector_slice_block;
pub use vector_slice_block::*;

mod vector_sort_block;
pub use vector_sort_block::*;

mod vector_index_block;
pub use vector_index_block::*;

mod i2c_output_block;
pub use i2c_output_block::*;

mod i2c_input_block;
pub use i2c_input_block::*;

mod bytes_unpack_block;
pub use bytes_unpack_block::*;

mod bytes_pack_block;
pub use bytes_pack_block::*;

mod serial_receive_block;
pub use serial_receive_block::*;

mod serial_transmit_block;
pub use serial_transmit_block::*;

mod json_load_block;
pub use json_load_block::*;

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

#[cfg(feature = "spi")]
mod spi_transmit_block;
#[cfg(feature = "spi")]
pub use spi_transmit_block::*;

#[cfg(feature = "spi")]
mod spi_receive_block;
#[cfg(feature = "spi")]
pub use spi_receive_block::*;

// mod character_display_block;
// pub use character_display_block::*;

// Declare all block modules and re-export them to make them importable from crate::blocks
cfg_if::cfg_if! {
  if #[cfg(feature = "std")] {

    mod fft_block;
    pub use fft_block::*;

    // mod lib_cam_block;
    // pub use lib_cam_block::*;

    mod system_time_block;
    pub use system_time_block::*;

    mod udp_receive_block;
    pub use udp_receive_block::*;

    mod udp_transmit_block;
    pub use udp_transmit_block::*;

    mod read_log_block;
    pub use read_log_block::*;
  }
}
