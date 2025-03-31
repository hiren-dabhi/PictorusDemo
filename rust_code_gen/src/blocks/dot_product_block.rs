use log::debug;

use crate::block_data::BlockData;

pub struct DotProductBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl DotProductBlock {
    pub fn new(name: &'static str, initial_data: &BlockData) -> DotProductBlock {
        DotProductBlock {
            name,
            data: initial_data.clone(),
        }
    }
    pub fn run(&mut self, input1: &BlockData, input2: &BlockData) {
        self.data = input1.dot(input2);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;

    #[test]
    fn test_vector_cross() {
        let initial_val = BlockData::new(1, 3, &[0., 0., 0.]);
        let mut dot_block = DotProductBlock::new("DotProduct", &initial_val);

        let input1 = BlockData::new(1, 3, &[1.0, 2.0, 3.0]);
        let input2 = BlockData::new(1, 3, &[4.0, 1.0, 0.0]);
        dot_block.run(&input1, &input2);

        assert!(dot_block.data.get_type() == BlockDataType::Scalar);
        assert!(dot_block.data.scalar() == 6.0);
    }
}
