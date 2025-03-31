use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum MinMaxEnum {
    Min,
    Max,
}

pub struct MinMaxBlock {
    name: &'static str,
    pub method: MinMaxEnum,
    pub data: BlockData,
}

impl MinMaxBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> MinMaxBlock {
        MinMaxBlock {
            name,
            method: method.parse().unwrap(),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        match self.method {
            MinMaxEnum::Min => {
                self.data = MinMaxBlock::_min(inputs);
            }
            MinMaxEnum::Max => {
                self.data = MinMaxBlock::_max(inputs);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn _min(inputs: &[&BlockData]) -> BlockData {
        let mut min_val = inputs[0].clone();
        for data in inputs.iter() {
            min_val = min_val.inf(data); // infimum (aka. componentwise min)
        }
        min_val
    }
    fn _max(inputs: &[&BlockData]) -> BlockData {
        let mut max_val = inputs[0].clone();
        for data in inputs.iter() {
            max_val = max_val.sup(data); // supremum (aka. componentwise max)
        }
        max_val
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloc::vec;

    #[test]
    fn test_minmax_block() {
        let ic = BlockData::from_scalar(0.0);
        let mut min_block = MinMaxBlock::new("MinMax1", &ic, "Min");
        let i1 = BlockData::from_scalar(1.0);
        let i2 = BlockData::from_scalar(-2.0);
        let i3 = BlockData::from_scalar(3.0);
        let inputs = vec![&i1, &i2, &i3];
        min_block.run(&inputs);
        assert_eq!(min_block.data.scalar(), -2.0);

        let mut max_block = MinMaxBlock::new("MinMax2", &ic, "Max");
        max_block.run(&inputs);
        assert_eq!(max_block.data.scalar(), 3.0);
    }

    #[test]
    fn test_minmax_vector() {
        let ic = BlockData::from_vector(&[0.0, 0.0, 0.0]);
        let mut min_block = MinMaxBlock::new("MinMax1", &ic, "Min");
        let i1 = BlockData::from_vector(&[1., 2., 3.]);
        let i2 = BlockData::from_vector(&[-2., 3., 1.]);
        let i3 = BlockData::from_vector(&[5., 0., -3.]);
        let inputs = vec![&i1, &i2, &i3];
        min_block.run(&inputs);

        assert_eq!(min_block.data, BlockData::from_vector(&[-2., 0., -3.]));

        let mut max_block = MinMaxBlock::new("MinMax2", &ic, "Max");
        max_block.run(&inputs);

        assert_eq!(max_block.data, BlockData::from_vector(&[5., 3., 3.]));
    }
}
