use alloc::collections::VecDeque;

use crate::block_data::{determine_data_type, BlockData, BlockDataType};
use log::debug;
use nalgebra::DMatrix;

pub struct SlidingWindowBlock {
    pub name: &'static str,
    pub queue: VecDeque<BlockData>,
    pub samples: usize,
    pub data: BlockData,
}

impl SlidingWindowBlock {
    pub fn new(name: &'static str, ic: &BlockData, samples: f64) -> SlidingWindowBlock {
        let n = samples as u8 as usize;
        let mut queue = VecDeque::with_capacity(n);
        let (sample_rows, mut sample_cols) = ic.size();
        sample_cols /= samples as u8 as usize;
        let mut ic_clone = ic.clone();
        for i in 0..n {
            queue.push_back(ic_clone.slice(0, i * sample_cols, sample_rows, sample_cols));
        }
        SlidingWindowBlock {
            name,
            samples: n,
            queue,
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, sample: &BlockData) {
        self.queue.push_back(sample.clone());
        self.queue.pop_front();
        let (new_data, new_type) = self.concatenate_queue();
        self.data = BlockData::from_data(new_data, new_type);
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn concatenate_queue(&self) -> (DMatrix<f64>, BlockDataType) {
        let rows = self.queue[0].nrows();
        let cols = self.queue.iter().map(|m| m.ncols()).sum();

        let mut result = DMatrix::zeros(rows, cols);

        let mut current_col = 0;
        for block in &self.queue {
            let data = block.get_data();
            for j in 0..data.ncols() {
                let col_vec = data.column(j);
                result.column_mut(current_col).copy_from(&col_vec);
                current_col += 1;
            }
        }

        (result, determine_data_type(rows, cols))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sliding_window_block() {
        let samples = 3.0;
        let mut block =
            SlidingWindowBlock::new("SW", &BlockData::from_element(1, 3, -1.0), samples);

        block.run(&BlockData::from_scalar(1.0));
        block.run(&BlockData::from_scalar(2.0));

        assert_eq!(block.data, BlockData::from_vector(&[-1.0, 1.0, 2.0]));

        block.run(&BlockData::from_scalar(3.0));
        block.run(&BlockData::from_scalar(4.0));

        assert_eq!(block.data, BlockData::from_vector(&[2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_sliding_window_block_vectors() {
        let samples = 3.0;
        let mut block = SlidingWindowBlock::new(
            "SW",
            &BlockData::new(1, 6, &[0., -1., -2., -3., -4., -5.]),
            samples,
        );

        block.run(&BlockData::from_vector(&[1.0, 2.0]));
        block.run(&BlockData::from_vector(&[3.0, 4.0]));

        assert_eq!(
            block.data,
            BlockData::from_vector(&[-4., -5., 1., 2., 3., 4.])
        );
    }

    #[test]
    fn test_sliding_window_block_matrices() {
        let samples = 3.0;
        let mut block = SlidingWindowBlock::new("SW", &BlockData::from_element(2, 6, 0.0), samples);

        block.run(&BlockData::new(2, 2, &[5., 6., 7., 8.]));
        block.run(&BlockData::new(2, 2, &[9., 10., 11., 12.]));

        let expected = BlockData::new(2, 6, &[0., 0., 5., 6., 9., 10., 0., 0., 7., 8., 11., 12.]);

        assert_eq!(block.data, expected);
    }
}
