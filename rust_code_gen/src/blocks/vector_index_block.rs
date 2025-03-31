use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use core::cmp::{max, min};
use log::debug;

use crate::block_data::BlockData;
use utils::parse_select_spec;
pub struct VectorIndexBlock {
    pub name: &'static str,
    pub data: Vec<BlockData>,
    pub indices: Vec<usize>,
}

impl VectorIndexBlock {
    pub fn new(
        name: &'static str,
        _initial_data: &BlockData,
        select_data: &[String],
    ) -> VectorIndexBlock {
        let data = vec![BlockData::from_scalar(0.0); max(select_data.len(), 1)];
        let indices = parse_select_spec(select_data);
        VectorIndexBlock {
            name,
            data,
            indices: indices.iter().map(|(_, i)| *i).collect(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        let data = input.get_data();
        for (i, lookup_index) in self
            .indices
            .iter()
            .enumerate()
            .take(min(data.len(), self.data.len()))
        {
            self.data[i].set_scalar(data[*lookup_index]);
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;
    use crate::block_data::BlockDataType;
    use crate::blocks::VectorMergeBlock;
    use alloc::vec;

    #[test]
    fn test_vector_index_block_scalar() {
        let initial_val = BlockData::from_scalar(-2.0);
        let indices = vec![
            "Scalar:1".to_string(),
            "Scalar:3".to_string(),
            "Scalar:7".to_string(),
            "Scalar:5".to_string(),
        ];
        let mut index_block = VectorIndexBlock::new("VectorIndex", &initial_val, &indices);

        let signal1 = BlockData::from_scalar(1.0);
        let signal2 = BlockData::from_vector(&[2.0, 3.0, 4.0]);
        let signal3 = BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]);

        let mut vector_block = VectorMergeBlock::new("VectorMerge", &initial_val);
        vector_block.run(&[&signal1, &signal2, &signal3]);

        index_block.run(&vector_block.data);

        assert!(index_block.data.len() == 4);

        for data in index_block.data.iter() {
            assert_eq!(data.get_type(), BlockDataType::Scalar);
        }

        assert!(index_block.data[0].scalar() == 2.0);
        assert!(index_block.data[1].scalar() == 4.0);
        assert!(index_block.data[2].scalar() == 8.0);
        assert!(index_block.data[3].scalar() == 6.0);
    }
    #[test]
    fn test_vector_index_block_matrix() {
        let initial_val = BlockData::from_scalar(-2.0);
        let indices = vec!["Scalar:1".to_string(), "Scalar:3".to_string()];

        let mut index_block = VectorIndexBlock::new("VectorIndex", &initial_val, &indices);

        let signal = BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]);
        index_block.run(&signal);

        // Hrmmm need to think about this. Indexing is column-major...
        assert_eq!(index_block.data[0].scalar(), 7.0);
        assert_eq!(index_block.data[1].scalar(), 8.0);
    }

    #[test]
    fn test_vector_index_block_input_too_short() {
        let initial_val = BlockData::from_scalar(-2.0);
        let indices = vec!["Scalar:0".to_string(), "Scalar:1".to_string()];

        let mut index_block = VectorIndexBlock::new("VectorIndex", &initial_val, &indices);

        let signal = BlockData::new(1, 1, &[5.0]);
        index_block.run(&signal);

        assert_eq!(index_block.data[0].scalar(), 5.0);
        assert_eq!(index_block.data[1].scalar(), 0.0);
    }
}
