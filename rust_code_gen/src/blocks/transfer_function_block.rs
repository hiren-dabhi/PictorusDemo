use alloc::collections::VecDeque;

use log::debug;

use crate::block_data::BlockData;

pub struct TransferFunctionBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub numerators: BlockData,
    pub denominators: BlockData,
    pub samples: VecDeque<BlockData>,
    pub n: usize,
}

impl TransferFunctionBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        numerators: &BlockData,
        denominators: &BlockData,
    ) -> TransferFunctionBlock {
        let n: usize = core::cmp::max(numerators.len(), denominators.len());
        let mut samples = VecDeque::with_capacity(n);
        for _idx in 0..n + 1 {
            samples.push_back(ic.clone());
        }
        TransferFunctionBlock {
            name,
            data: ic.clone(),
            numerators: numerators.clone(),
            denominators: denominators.clone(),
            samples,
            n,
        }
    }
    pub fn run(&mut self, sample: &BlockData) {
        self.samples.push_front(sample.clone());
        self.samples.pop_back();

        // TODO: Use something like this instead: https://docs.rs/automatica/latest/automatica/
        let mut denom_sum = BlockData::zeros_sizeof(&self.data);
        for i in 1..self.n {
            if i < self.denominators.len() {
                denom_sum += &(self.samples[i].clone() * self.denominators[i]);
            }
        }
        self.samples[0] -= &denom_sum;
        self.samples[0] /= self.denominators[0];
        let mut data = BlockData::zeros_sizeof(&self.data);
        for i in 0..self.n {
            if i < self.numerators.len() {
                data += &(&self.samples[i] * self.numerators[i]);
            }
        }
        self.data = data;
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_transfer_function_block() {
        let numerators = BlockData::from_vector(&[1.0, -2.0]);
        let denominators = BlockData::from_vector(&[3.0]);

        let mut signal = BlockData::from_scalar(0.0);

        let mut block = TransferFunctionBlock::new("TF1", &signal, &numerators, &denominators);

        // TODO: This is just a functional test, need to do some maths here to prove out block
        block.run(&signal);
        assert_eq!(block.data.scalar(), 0.0);

        signal.set_scalar(1.0);
        block.run(&signal);
        assert_relative_eq!(block.data.scalar(), 0.333, max_relative = 0.01);
    }

    #[test]
    fn test_vectorized_transfer_function_block() {
        let numerators = BlockData::from_vector(&[1.0, -2.0]);
        let denominators = BlockData::from_vector(&[3.0]);

        let mut signal = BlockData::from_vector(&[0.0, 0.0]);

        let mut block = TransferFunctionBlock::new("TF1", &signal, &numerators, &denominators);

        // TODO: This is just a functional test, need to do some maths here to prove out block
        block.run(&signal);
        assert_relative_eq!(
            block.data,
            BlockData::from_vector(&[0.0, 0.0]),
            max_relative = 0.01
        );

        signal = BlockData::from_vector(&[0.0, 1.0]);
        block.run(&signal);
        assert_relative_eq!(
            block.data,
            BlockData::from_vector(&[0.0, 0.333]),
            max_relative = 0.01
        );
    }
}
