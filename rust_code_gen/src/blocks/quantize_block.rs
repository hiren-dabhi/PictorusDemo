use log::debug;
use num_traits::Float;

use crate::block_data::BlockData;

pub struct QuantizeBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub interval: BlockData,
}

impl QuantizeBlock {
    pub fn new(name: &'static str, ic: &BlockData, interval: f64) -> QuantizeBlock {
        QuantizeBlock {
            name,
            data: ic.clone(),
            interval: BlockData::scalar_sizeof(interval, ic),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        let input_divided_interval = input / &self.interval;
        let rounded = input_divided_interval.map(Float::round);
        self.data = rounded.component_mul(&self.interval);

        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantize_block() {
        let interval: f64 = 0.5;
        let ic = BlockData::from_vector(&[0.0, 0.5, 1.0, 2.0]);
        let mut block = QuantizeBlock::new("Quantize1", &ic, interval);
        assert_eq!(block.data, ic);

        block.run(&BlockData::from_vector(&[0.24, 0.25, 0.51, 0.75]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.5, 0.5, 1.0]));
    }
}
