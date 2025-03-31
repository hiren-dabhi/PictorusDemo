use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum LogicalEnum {
    And,
    Or,
    Nor,
    Nand,
}

pub struct LogicalBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: LogicalEnum,
}

impl LogicalBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> LogicalBlock {
        LogicalBlock {
            name,
            data: ic.clone(),
            method: method.parse().unwrap(),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        match self.method {
            LogicalEnum::And => {
                self.data = self.run_and(inputs);
            }
            LogicalEnum::Or => {
                self.data = self.run_or(inputs);
            }
            LogicalEnum::Nor => {
                self.data = self.run_nor(inputs);
            }
            LogicalEnum::Nand => {
                self.data = self.run_nand(inputs);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
    pub fn run_and(&self, inputs: &[&BlockData]) -> BlockData {
        let mut data = BlockData::ones_sizeof(&self.data);
        for input_data in inputs.iter() {
            data = data.and(input_data);
        }
        data
    }
    pub fn run_or(&self, inputs: &[&BlockData]) -> BlockData {
        let mut data = BlockData::zeros_sizeof(&self.data);
        for input_data in inputs.iter() {
            data = data.or(input_data);
        }
        data
    }
    pub fn run_nor(&self, inputs: &[&BlockData]) -> BlockData {
        let data = self.run_or(inputs);
        BlockData::ones_sizeof(&self.data) - &data
    }
    pub fn run_nand(&self, inputs: &[&BlockData]) -> BlockData {
        let data = self.run_and(inputs);
        BlockData::ones_sizeof(&self.data) - &data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_block_and_method() {
        let ic = BlockData::from_scalar(1.234);
        let mut block = LogicalBlock::new("Logical1", &ic, "And");

        // All zero aka false inputs = false output
        block.run(&[
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // Some zero inputs = false output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // All non-zero inputs = true output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // Even floats and negative data!
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(-2.0),
            &BlockData::from_scalar(3.5),
            &BlockData::from_scalar(1234.566),
            &BlockData::from_scalar(-12.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_logical_block_or_method() {
        let ic = BlockData::from_scalar(1.234);
        let mut block = LogicalBlock::new("Logical2", &ic, "Or");

        // All zero aka false inputs = false output
        block.run(&[
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // Some zero inputs = true output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // All non-zero inputs = true output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // Even floats and negative data!
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(-2.0),
            &BlockData::from_scalar(3.5),
            &BlockData::from_scalar(1234.566),
            &BlockData::from_scalar(-12.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_logical_block_nor_method() {
        let ic = BlockData::from_scalar(1.234);
        let mut block = LogicalBlock::new("Logical3", &ic, "Nor");

        // These tests should be the opposite results of the OR tests

        // All zero aka false inputs = true output
        block.run(&[
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // Some zero inputs = false output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // All non-zero inputs = false output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // Even floats and negative data!
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(-2.0),
            &BlockData::from_scalar(3.5),
            &BlockData::from_scalar(1234.566),
            &BlockData::from_scalar(-12.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);
    }

    #[test]
    fn test_logical_block_nand_method() {
        let ic = BlockData::from_scalar(1.234);
        let mut block = LogicalBlock::new("Logical4", &ic, "Nand");

        // These tests should be the opposite results of the AND tests

        // All zero aka false inputs = true output
        block.run(&[
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(0.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // Some zero inputs = true output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(0.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 1.0);

        // All non-zero inputs = false output
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(1.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);

        // Even floats and negative data!
        block.run(&[
            &BlockData::from_scalar(1.0),
            &BlockData::from_scalar(-2.0),
            &BlockData::from_scalar(3.5),
            &BlockData::from_scalar(1234.566),
            &BlockData::from_scalar(-12.0),
        ]);
        assert_eq!(block.data.scalar(), 0.0);
    }
}
