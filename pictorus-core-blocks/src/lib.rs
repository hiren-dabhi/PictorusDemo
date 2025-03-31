//! Set of blocks that do not depend on `std` or `alloc`
#![no_std]

#[cfg(test)]
extern crate std;

// Add new core blocks here
mod abs_block;
pub use abs_block::AbsBlock;

mod aggregate_block;
pub use aggregate_block::AggregateBlock;

mod app_time_block;
pub use app_time_block::AppTimeBlock;

mod arg_min_max_block;
pub use arg_min_max_block::ArgMinMaxBlock;

mod bias_block;
pub use bias_block::BiasBlock;

mod bit_shift_block;
pub use bit_shift_block::BitShiftBlock;

mod bytes_literal_block;
pub use bytes_literal_block::BytesLiteralBlock;

mod clamp_block;
pub use clamp_block::ClampBlock;

mod comparison_block;
pub use comparison_block::ComparisonBlock;

mod compare_to_value_block;
pub use compare_to_value_block::CompareToValueBlock;

mod constant_block;
pub use constant_block::ConstantBlock;

mod counter_block;
pub use counter_block::CounterBlock;

mod cross_product_block;
pub use cross_product_block::CrossProductBlock;

mod gain_block;
pub use gain_block::GainBlock;

mod ramp_block;
pub use ramp_block::RampBlock;

mod random_number_block;
pub use random_number_block::RandomNumberBlock;

mod sawtoothwave_block;
pub use sawtoothwave_block::SawtoothwaveBlock;

mod sinewave_block;
pub use sinewave_block::SinewaveBlock;

mod squarewave_block;
pub use squarewave_block::SquarewaveBlock;

mod sum_block;
pub use sum_block::SumBlock;

mod trianglewave_block;
pub use trianglewave_block::TrianglewaveBlock;
