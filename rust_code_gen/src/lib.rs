#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]
#![deny(clippy::undocumented_unsafe_blocks)]
extern crate alloc;

pub mod loggers;
pub use utils::block_data;
pub mod blocks;
pub use utils;

#[cfg(all(test, feature = "std"))]
mod tests {

    use crate::block_data::BlockData;
    use crate::blocks::{
        constant_block::ConstantBlock, gain_block::GainBlock, product_block::ProductBlock,
        sum_block::SumBlock,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_app_interface() {
        // Create a simple AppInterface with a contant and gain block
        struct AppInterface {
            constant: ConstantBlock,
            gain: GainBlock,
        }

        impl AppInterface {
            fn new() -> AppInterface {
                let constant = ConstantBlock::new("Constant1", &BlockData::from_scalar(3.0));
                let gain = GainBlock::new("Gain1", &BlockData::from_scalar(0.0), 2.0);
                AppInterface { constant, gain }
            }
            fn run(&mut self) {
                self.gain.run(&self.constant.data);
            }
        }

        // Run the interface for one iteration and verify the gain block multiplied the constant block input
        let mut app_interface: AppInterface = AppInterface::new();
        app_interface.run();

        assert_eq!(app_interface.gain.data.scalar(), 6.0);
    }

    #[test]
    fn test_vectors() {
        // Create a simple AppInterface with two (vectorized) constants, a gain block, and a sum,
        // to demonstate vector interactions
        struct AppInterface {
            constant1: ConstantBlock,
            constant2: ConstantBlock,
            gain: GainBlock,
            sum: SumBlock,
            product: ProductBlock,
        }

        impl AppInterface {
            fn new() -> AppInterface {
                let constant1 =
                    ConstantBlock::new("Constant1", &BlockData::from_vector(&[3.0, 2.0, 1.0]));
                let constant2 =
                    ConstantBlock::new("Constant2", &BlockData::from_vector(&[5.0, -4.0, 3.0]));
                let gain = GainBlock::new("Gain1", &BlockData::from_vector(&[0.0, 0.0, 0.0]), 2.0);
                let sum = SumBlock::new(
                    "Sum1",
                    &BlockData::from_vector(&[0.0, 0.0, 0.0]),
                    &BlockData::from_vector(&[1.0, -1.0]),
                );
                let product = ProductBlock::new(
                    "ProductBlock1",
                    &BlockData::from_vector(&[0.0, 0.0, 0.0]),
                    &BlockData::from_vector(&[1.0, -1.0]),
                    "ComponentWise",
                );
                AppInterface {
                    constant1,
                    constant2,
                    gain,
                    sum,
                    product,
                }
            }
            fn run(&mut self) {
                self.gain.run(&self.constant1.data);
                self.sum.run(&[&self.gain.data, &self.constant2.data]);
                self.product.run(&[&self.constant1.data, &self.sum.data]);
            }
        }

        let mut app_interface: AppInterface = AppInterface::new();
        app_interface.run();
        assert_eq!(
            app_interface.sum.data,
            BlockData::from_vector(&[1.0, 8.0, -1.0])
        );
        assert_eq!(
            app_interface.product.data,
            BlockData::from_vector(&[3.0 / 1.0, 2.0 / 8.0, 1.0 / -1.0])
        );
    }

    #[test]
    fn test_matrices() {
        // Create a simple AppInterface with two (matrix) constants, a gain block, and a sum,
        // to demonstate matrix interactions
        struct AppInterface {
            constant1: ConstantBlock,
            constant2: ConstantBlock,
            gain: GainBlock,
            sum: SumBlock,
            product: ProductBlock,
        }

        impl AppInterface {
            fn new() -> AppInterface {
                let constant1 =
                    ConstantBlock::new("Constant1", &BlockData::new(2, 2, &[3.0, 2.0, 1.0, 1.0]));
                let constant2 =
                    ConstantBlock::new("Constant2", &BlockData::new(2, 2, &[5.0, -4.0, 3.0, 1.0]));
                let gain = GainBlock::new("Gain1", &BlockData::new(2, 2, &[0., 0., 0., 0.]), 2.0);
                let sum = SumBlock::new(
                    "Sum1",
                    &BlockData::new(2, 2, &[0., 0., 0., 0.]),
                    &BlockData::from_vector(&[1.0, -1.0]),
                );
                let product = ProductBlock::new(
                    "ProductBlock1",
                    &BlockData::new(2, 2, &[0., 0., 0., 0.]),
                    &BlockData::from_vector(&[1.0, -1.0]),
                    "ComponentWise",
                );
                AppInterface {
                    constant1,
                    constant2,
                    gain,
                    sum,
                    product,
                }
            }
            fn run(&mut self) {
                self.gain.run(&self.constant1.data);
                self.sum.run(&[&self.gain.data, &self.constant2.data]);
                self.product.run(&[&self.constant1.data, &self.sum.data]);
            }
        }

        let mut app_interface: AppInterface = AppInterface::new();
        app_interface.run();
        assert_eq!(
            app_interface.sum.data,
            BlockData::new(2, 2, &[1.0, 8.0, -1.0, 1.0])
        );
        assert_eq!(
            app_interface.product.data,
            BlockData::new(2, 2, &[3.0 / 1.0, 2.0 / 8.0, 1.0 / -1.0, 1.0 / 1.0])
        );
    }
}
