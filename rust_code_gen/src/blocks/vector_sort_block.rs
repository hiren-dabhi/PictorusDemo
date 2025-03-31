use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum VectorSortEnum {
    Ascending,
    Descending,
}

pub struct VectorSortBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: VectorSortEnum,
}

impl VectorSortBlock {
    pub fn new(name: &'static str, _initial_data: &BlockData, method: &str) -> VectorSortBlock {
        VectorSortBlock {
            name,
            data: _initial_data.clone(),
            method: method.parse().unwrap(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = match self.method {
            VectorSortEnum::Ascending => input.sorted(true),
            VectorSortEnum::Descending => input.sorted(false),
        };

        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_sort_ascending() {
        let initial_val = BlockData::from_scalar(0.0);
        let mut slice_block = VectorSortBlock::new("VectorSort", &initial_val, "Ascending");

        slice_block.run(&BlockData::from_vector(&[
            3., 6., 9., 7., 5., 2., 1., 10., 8., 4.,
        ]));

        assert!(
            slice_block.data == BlockData::from_vector(&[1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,])
        );
    }

    #[test]
    fn test_vector_sort_descending() {
        let initial_val = BlockData::from_scalar(0.0);
        let mut slice_block = VectorSortBlock::new("VectorSort", &initial_val, "Descending");

        slice_block.run(&BlockData::from_vector(&[
            3., 6., 9., 7., 5., 2., 1., 10., 8., 4.,
        ]));

        assert!(
            slice_block.data == BlockData::from_vector(&[10., 9., 8., 7., 6., 5., 4., 3., 2., 1.,])
        );
    }
}
