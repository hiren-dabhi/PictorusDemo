use crate::block_data::BlockData;

pub struct DataWriteBlock {
    pub data: BlockData,
}

impl DataWriteBlock {
    pub fn new(_: &str) -> DataWriteBlock {
        DataWriteBlock {
            data: BlockData::from_scalar(0.0), // TODO: IC
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_block() {
        let _block = DataWriteBlock::new("DataWriteBlock");
    }
}
