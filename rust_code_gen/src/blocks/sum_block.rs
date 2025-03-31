use log::debug;

use crate::block_data::BlockData;

pub struct SumBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub signs: BlockData,
}

impl SumBlock {
    pub fn new(name: &'static str, initial_data: &BlockData, signs: &BlockData) -> SumBlock {
        SumBlock {
            name,
            data: initial_data.clone(),
            signs: signs.clone(),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        let mut sum = BlockData::zeros_sizeof(&self.data);
        for (i, block) in inputs.iter().enumerate() {
            let signed_val = *block * self.signs[i];
            sum += &signed_val;
        }
        self.data = sum;

        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_block_scalar() {
        let initial_val = BlockData::from_scalar(-2.0);
        let signs = BlockData::from_vector(&[1.0, -1.0, 1.0]);
        let mut block = SumBlock::new("Sum1", &initial_val, &signs);

        let signal1 = BlockData::from_scalar(1.0);
        let signal2 = BlockData::from_scalar(2.0);
        let signal3 = BlockData::from_scalar(3.0);

        block.run(&[&signal1, &signal2, &signal3]);

        assert_eq!(block.data.scalar(), 2.0);
    }

    #[test]
    fn test_sum_block_vector() {
        let initial_val = BlockData::from_vector(&[0.0, 0.0, 0.0]);
        let signs = BlockData::from_vector(&[1.0, -1.0, 1.0]);
        let mut block = SumBlock::new("Sum1", &initial_val, &signs);

        let signal1 = BlockData::from_vector(&[1.0, 2.0, 1.0]);
        let signal2 = BlockData::from_vector(&[2.0, 3.0, 4.0]);
        let signal3 = BlockData::from_vector(&[-5.0, -1.0, 1.0]);

        block.run(&[&signal1, &signal2, &signal3]);
        assert_eq!(block.data, BlockData::from_vector(&[-6.0, -2.0, -2.0]));
    }

    #[test]
    fn test_sum_block_matrix() {
        let initial_val = BlockData::new(2, 2, &[0.0, 0.0, 0.0, 0.0]);
        let signs = BlockData::from_vector(&[1.0, -1.0, 1.0]);
        let mut block = SumBlock::new("Sum1", &initial_val, &signs);

        let signal1 = BlockData::new(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let signal2 = BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]);
        let signal3 = BlockData::new(2, 2, &[9.0, 10.0, 11.0, 12.0]);

        block.run(&[&signal1, &signal2, &signal3]);
        /*
        0,0 = 1 - 5 + 9 = 5
        0,1 = 2 - 6 + 10 = 6
        1,0 = 3 - 7 + 11 = 7
        1,1 = 4 - 8 + 12 = 8
        */
        assert_eq!(block.data, BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]));
    }
    #[test]
    fn test_sum_block_scalar_and_matrix() {
        let initial_val = BlockData::new(2, 2, &[0.0, 0.0, 0.0, 0.0]);
        let signs = BlockData::from_vector(&[1.0, -1.0]);
        let mut block = SumBlock::new("Sum1", &initial_val, &signs);

        let signal1 = BlockData::from_scalar(2.0);
        let signal2 = BlockData::new(2, 2, &[9.0, 10.0, 11.0, 12.0]);

        block.run(&[&signal1, &signal2]);
        assert_eq!(block.data, BlockData::new(2, 2, &[-7.0, -8.0, -9.0, -10.0]));
    }
}
