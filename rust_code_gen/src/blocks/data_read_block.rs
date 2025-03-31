use crate::block_data::BlockData;

pub struct DataReadBlock {
    pub data: BlockData,
}

impl DataReadBlock {
    pub fn new(_: &str) -> DataReadBlock {
        DataReadBlock {
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
        let _block = DataReadBlock::new("DataReadBlock");
    }
}
