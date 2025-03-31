use alloc::string::{String, ToString};

use crate::block_data::BlockData;
use log::debug;

#[derive(strum::Display, strum::EnumString)]
pub enum IntegralEnum {
    Rectangle,
    Trapezoidal,
}

pub struct IntegralBlock {
    pub name: String,
    pub method: IntegralEnum,
    pub previous_sample: BlockData,
    pub upper_clamp_limit: BlockData,
    pub lower_clamp_limit: BlockData,
    pub data: BlockData,
}

impl IntegralBlock {
    pub fn new(name: &str, ic: &BlockData, clamp_limit: f64, method: &str) -> IntegralBlock {
        IntegralBlock {
            name: name.to_string(),
            method: method.parse().unwrap(),
            previous_sample: ic.clone(),
            upper_clamp_limit: BlockData::scalar_sizeof(clamp_limit, ic),
            lower_clamp_limit: BlockData::scalar_sizeof(-1.0 * clamp_limit, ic),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, timestep_s: f64, sample: &BlockData, reset: Option<&BlockData>) {
        let delta = match self.method {
            IntegralEnum::Rectangle => timestep_s * sample,
            IntegralEnum::Trapezoidal => timestep_s * 0.5 * (sample + &self.previous_sample),
        };

        self.data += &delta;
        let over_clamp_limit = self.data.gt(&self.upper_clamp_limit);
        self.data
            .component_set(&over_clamp_limit, &self.upper_clamp_limit);
        let under_clamp_limit = self.data.lt(&self.lower_clamp_limit);
        self.data
            .component_set(&under_clamp_limit, &self.lower_clamp_limit);
        self.maybe_reset(reset);
        self.previous_sample = sample.clone();

        debug!("{} data: {:?}", self.name, self.data);
    }
    pub fn maybe_reset(&mut self, reset: Option<&BlockData>) {
        // If reset is non-zero, multiply the integral by zero to reset it.
        if let Some(reset) = reset {
            self.data.maybe_reset(reset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::sinewave_block::SinewaveBlock;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    #[test]
    fn test_integral_block() {
        let ic = BlockData::from_scalar(0.0);
        let clamp_limit = 20.0;
        let mut block = IntegralBlock::new("Integral1", &ic, clamp_limit, "Trapezoidal");

        let mut sin_block = SinewaveBlock::new("TestSin", 1., 1., 0., 0.);
        // Cosine is sinewave offset with pi/2 phase shift
        let mut cos_block = SinewaveBlock::new("TestCos", 1., 1., PI / 2.0, 0.);

        let dt = 0.1;
        let mut time = 0.0;
        while time < 10.0 {
            time += dt;
            block.run(dt, &cos_block.data, None);
            // Integral of cosine is sine, with a small offset and allowing discrete tolerance.
            assert_relative_eq!(
                block.data.scalar(),
                sin_block.data.scalar() + 0.05,
                max_relative = 0.01
            );
            sin_block.run(time);
            cos_block.run(time);
        }

        // Reset with any input value
        block.run(
            dt,
            &BlockData::from_scalar(1000000.0),
            Some(&BlockData::from_scalar(1.0)),
        );
        assert_relative_eq!(block.data.scalar(), BlockData::from_scalar(0.0).scalar());
    }
    #[test]
    fn test_integral_block_vector() {
        let ic = BlockData::from_vector(&[-5.0, 1.0, 15.0]);
        let clamp_limit = 20.0;
        let mut block = IntegralBlock::new("Integral1", &ic, clamp_limit, "Rectangle");
        let reset = BlockData::from_vector(&[0.0, 1.0, 0.0]);

        block.run(
            1.0,
            &BlockData::from_vector(&[1.0, 1.0, 15.0]),
            Some(&reset),
        );
        assert_eq!(block.data, BlockData::from_vector(&[-4.0, 0.0, 20.0]));
    }
}
