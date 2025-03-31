use log::debug;

use crate::block_data::BlockData;

pub struct VectorNormBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl VectorNormBlock {
    pub fn new(name: &'static str, _initial_data: &BlockData) -> VectorNormBlock {
        VectorNormBlock {
            name,
            data: BlockData::from_scalar(0.0),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data.set_scalar(input.norm());
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;

    #[test]
    fn test_vector_norm() {
        let initial_val = BlockData::new(1, 2, &[0., 0.]);
        let mut norm_block = VectorNormBlock::new("VectorNorm", &initial_val);

        let input = BlockData::new(1, 2, &[3.0, 4.0]);
        norm_block.run(&input);

        assert!(norm_block.data.get_type() == BlockDataType::Scalar);
        assert!(norm_block.data.get_data() == BlockData::from_scalar(5.0).get_data());
    }
}
