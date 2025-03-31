use log::debug;

use crate::block_data::BlockData;

pub struct GainBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub gain: BlockData,
}

impl GainBlock {
    pub fn new(name: &'static str, ic: &BlockData, gain: f64) -> GainBlock {
        GainBlock {
            name,
            data: ic.clone(),
            gain: BlockData::scalar_sizeof(gain, ic),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.component_mul(&self.gain);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_block() {
        let mut gain_block =
            GainBlock::new("Gain1", &BlockData::from_vector(&[0.0, 0.0, 0.0]), 2.0);

        gain_block.run(&BlockData::from_vector(&[2., -3., -4.]));
        assert_eq!(gain_block.data, BlockData::from_vector(&[4., -6., -8.]));
    }
}
