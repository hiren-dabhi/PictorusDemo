use log::debug;

use crate::block_data::BlockData;

pub struct VectorSliceBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub row0: usize,
    pub col0: usize,
    pub rows: usize,
    pub cols: usize,
}

impl VectorSliceBlock {
    pub fn new(
        name: &'static str,
        _initial_data: &BlockData,
        row0: f64,
        col0: f64,
        shape: &BlockData,
    ) -> VectorSliceBlock {
        VectorSliceBlock {
            name,
            data: BlockData::from_element(shape.at(0) as usize, shape.at(1) as usize, 0.0),
            row0: row0 as usize,
            col0: col0 as usize,
            rows: shape.at(0) as usize,
            cols: shape.at(1) as usize,
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        if self.row0 > input.nrows() || self.row0 + self.rows > input.nrows() {
            panic!(
                "{} Slice rows exceed input dimensions {:?}!",
                self.name,
                input.size()
            );
        }
        if self.col0 > input.ncols() || self.col0 + self.cols > input.ncols() {
            panic!(
                "{} Slice cols exceed input dimensions {:?}!",
                self.name,
                input.size()
            );
        }

        self.data = input
            .clone()
            .slice(self.row0, self.col0, self.rows, self.cols);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_slice_block() {
        let initial_val = BlockData::from_scalar(0.0);
        let shape = BlockData::new(1, 2, &[3., 3.]);
        let mut slice_block = VectorSliceBlock::new("VectorIndex", &initial_val, 1.0, 1.0, &shape);

        slice_block.run(&BlockData::from_row_slice(
            5,
            5,
            &[
                1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.,
                19., 20., 21., 22., 23., 24., 25.,
            ],
        ));

        assert!(
            slice_block.data
                == BlockData::from_row_slice(3, 3, &[7., 8., 9., 12., 13., 14., 17., 18., 19.,])
        );
    }

    #[test]
    #[should_panic]
    fn test_vector_slice_block_panics_bad_dims() {
        let initial_val = BlockData::from_scalar(0.0);
        let shape = BlockData::new(1, 2, &[3., 3.]);
        let mut slice_block = VectorSliceBlock::new("VectorIndex", &initial_val, 1.0, 1.0, &shape);

        slice_block.run(&BlockData::from_row_slice(
            3,
            3,
            &[1., 2., 3., 6., 7., 8., 11., 12., 13.],
        ));
    }
}
