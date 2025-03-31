use log::debug;

use crate::block_data::BlockData;

pub struct TransposeBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl TransposeBlock {
    pub fn new(name: &'static str, initial_data: &BlockData) -> TransposeBlock {
        TransposeBlock {
            name,
            data: initial_data.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.transpose();
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_tranpose() {
        let initial_val = BlockData::new(2, 3, &[0., 0., 0., 0., 0., 0.]);
        let mut reshape_block = TransposeBlock::new("VectorTranspose", &initial_val);

        let input = BlockData::new(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        reshape_block.run(&input);

        assert_eq!(
            reshape_block.data,
            BlockData::new(2, 3, &[1.0, 3.0, 5.0, 2.0, 4.0, 6.0])
        );
    }
}
