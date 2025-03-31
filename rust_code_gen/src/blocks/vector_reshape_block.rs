use log::debug;

use crate::block_data::BlockData;

pub struct VectorReshapeBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub shape: (usize, usize),
}

impl VectorReshapeBlock {
    pub fn new(
        name: &'static str,
        initial_data: &BlockData,
        shape: &BlockData,
    ) -> VectorReshapeBlock {
        VectorReshapeBlock {
            name,
            data: initial_data.clone(),
            shape: (shape.at(0) as usize, shape.at(1) as usize),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = BlockData::new(self.shape.0, self.shape.1, input.get_data().as_slice());
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;

    #[test]
    fn test_vector_reshape() {
        let initial_val = BlockData::new(2, 3, &[0., 0., 0., 0., 0., 0.]);
        let shape = BlockData::new(1, 2, &[2., 3.]);
        let mut reshape_block = VectorReshapeBlock::new("VectorReshape", &initial_val, &shape);

        let input = BlockData::new(1, 6, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        reshape_block.run(&input);

        assert!(reshape_block.data.get_type() == BlockDataType::Matrix);
        assert!(reshape_block.data.size() == (2, 3));
        assert!(
            reshape_block.data.get_data()
                == BlockData::new(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).get_data()
        );
    }
}
