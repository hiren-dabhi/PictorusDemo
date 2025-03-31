use log::debug;

use crate::block_data::BlockData;

pub struct DeterminantBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl DeterminantBlock {
    pub fn new(name: &'static str, initial_data: &BlockData) -> DeterminantBlock {
        if initial_data.nrows() != initial_data.ncols() {
            panic!("Cannot compute determinant of non-square matrix!");
        }
        DeterminantBlock {
            name,
            data: initial_data.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.determinant();
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;

    #[test]
    fn test_vector_determinant() {
        let initial_val = BlockData::new(2, 2, &[0., 0., 0., 0.]);
        let mut det_block = DeterminantBlock::new("VectorDeterminant", &initial_val);

        let input = BlockData::new(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        det_block.run(&input);

        assert!(det_block.data.scalar() == -2.0);
        assert!(det_block.data.get_type() == BlockDataType::Scalar);
    }
}
