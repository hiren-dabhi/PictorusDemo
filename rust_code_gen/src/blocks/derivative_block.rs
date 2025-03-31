use alloc::string::{String, ToString};

use log::debug;

use alloc::collections::VecDeque;

use crate::block_data::BlockData;

pub enum DerivativeEnum {
    NSamples,
}

pub struct DerivativeBlock {
    pub name: String,
    pub method: DerivativeEnum,
    pub previous_samples: VecDeque<BlockData>,
    pub max_samples: usize,
    pub data: BlockData,
}

impl DerivativeBlock {
    pub fn new(name: &str, ic: &BlockData, max_samples: f64) -> DerivativeBlock {
        DerivativeBlock {
            name: name.to_string(),
            method: DerivativeEnum::NSamples,
            max_samples: max_samples as u8 as usize,
            previous_samples: VecDeque::with_capacity(max_samples as u8 as usize),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, timestep_s: f64, sample: &BlockData) {
        match self.method {
            DerivativeEnum::NSamples => {
                self._run_n_samples(timestep_s, sample);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn _run_n_samples(&mut self, timestep_s: f64, sample: &BlockData) {
        self.previous_samples.push_back(sample.clone());
        if self.previous_samples.len() < self.max_samples {
            return;
        }

        let oldest = self.previous_samples.front().unwrap();
        let newest = self.previous_samples.get(self.max_samples - 1).unwrap();
        self.data = (newest - oldest) / (((self.max_samples - 1) as f64) * timestep_s);
        self.previous_samples.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative_block() {
        let samples = 2.0;
        let ic = BlockData::from_vector(&[0.0, 1.0]);

        let mut block = DerivativeBlock::new("Derivative1", &ic, samples);

        let dt = 0.5;
        block.run(dt, &BlockData::from_vector(&[0.0, 0.0]));
        // Needs 2 samples before computing derivative (maintains IC)
        assert_eq!(block.data, BlockData::from_vector(&[0., 1.]));

        block.run(dt, &BlockData::from_vector(&[2.0, 4.0]));
        assert_eq!(block.data, BlockData::from_vector(&[4., 8.]));

        block.run(dt, &BlockData::from_vector(&[6.0, 12.0]));
        assert_eq!(block.data, BlockData::from_vector(&[8., 16.]));

        block.run(dt, &BlockData::from_vector(&[0.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[-12., -24.]));
    }
}
