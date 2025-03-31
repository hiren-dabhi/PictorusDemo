use alloc::vec::Vec;

use log::debug;

use crate::block_data::BlockData;

pub struct VectorMergeBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl VectorMergeBlock {
    pub fn new(name: &'static str, initial_data: &BlockData) -> VectorMergeBlock {
        VectorMergeBlock {
            name,
            data: initial_data.clone(),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        let mut vector_vals: Vec<f64> = Vec::new();
        for block in inputs.iter() {
            for i in 0..block.nrows() {
                for j in 0..block.ncols() {
                    vector_vals.push(block.at_rc(i, j));
                }
            }
        }
        self.data = BlockData::from_row_slice(1, vector_vals.len(), &vector_vals);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;

    #[test]
    fn test_vector_merge_block_scalar() {
        // Should be able to pass in scalars, vectors, or matrices,
        // and get back a flattened vector
        let initial_val = BlockData::from_scalar(-2.0);
        let mut block = VectorMergeBlock::new("VectorMerge", &initial_val);

        let signal1 = BlockData::from_scalar(1.0);
        let signal2 = BlockData::from_vector(&[2.0, 3.0, 4.0]);
        let signal3 = BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]);

        block.run(&[&signal1, &signal2, &signal3]);

        assert_eq!(block.data.get_type(), BlockDataType::Vector);

        let expected = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert!(block.data.vector().iter().eq(expected.iter()));
    }

    // TODO: Do we want to be able to specify the vector dimensions in params?
}
