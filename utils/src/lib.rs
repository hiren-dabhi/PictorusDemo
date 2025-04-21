#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod utils;
pub use utils::*;

pub mod block_data;
mod block_ops;
pub use block_data::*;

pub mod runtime_context;
pub use runtime_context::RuntimeContext;

pub mod execution_controller;
pub use execution_controller::ExecutionController;

pub mod stale_tracker;
pub use stale_tracker::StaleTracker;

pub mod byte_data;

pub mod timing;

pub trait IsValid {
    fn is_valid(&self, app_time_s: f64) -> BlockData;
}
