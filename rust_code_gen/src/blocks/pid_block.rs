use alloc::format;
use alloc::string::{String, ToString};

use log::debug;

use crate::block_data::BlockData;

use super::{DerivativeBlock, IntegralBlock};

pub struct PidBlock {
    pub name: String,
    pub kp: BlockData,
    pub ki: BlockData,
    pub kd: BlockData,
    pub integrator: IntegralBlock,
    pub derivative: DerivativeBlock,
    pub data: BlockData,
}

impl PidBlock {
    pub fn new(
        name: &str,
        ic: &BlockData,
        kp: f64,
        ki: f64,
        kd: f64,
        i_max: f64,
        kd_samples: f64,
    ) -> PidBlock {
        PidBlock {
            name: name.to_string(),
            kp: kp * BlockData::ones_sizeof(ic),
            ki: ki * BlockData::ones_sizeof(ic),
            kd: kd * BlockData::ones_sizeof(ic),
            integrator: IntegralBlock::new(
                format!("{}_integral", name).as_str(),
                &(ic.clone()),
                i_max,
                "Rectangle",
            ),
            derivative: DerivativeBlock::new(
                format!("{}_derivative", name).as_str(),
                &BlockData::zeros_sizeof(ic),
                kd_samples,
            ),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, timestep_s: f64, error: &BlockData, reset: Option<&BlockData>) {
        // Run integrator with error
        self.integrator
            .run(timestep_s, &self.ki.component_mul(error), None);
        self.integrator.maybe_reset(reset);

        // Run derivative run error
        self.derivative.run(timestep_s, error);
        // Add them all up!
        self.data = &self.kp.component_mul(error)
            + &self.integrator.data
            + &self.kd.component_mul(&self.derivative.data);

        debug!(
            "{}: {:?}  (P: {:?} I: {:?} D: {:?})",
            self.name,
            self.data,
            &self.kp * error,
            self.integrator.data,
            &self.kd * &self.derivative.data,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pid_block() {
        let kp = 2.0;
        let ki = 0.0;
        let kd = 0.0;
        let i_max = 0.0;
        let kd_samples = 2.0;
        let ic = BlockData::from_scalar(0.0);

        let mut p_block = PidBlock::new("PID1", &ic, kp, ki, kd, i_max, kd_samples);

        let dt = 1.0;

        // Output should just be double the input
        p_block.run(dt, &BlockData::from_scalar(1.0), None);
        assert_eq!(p_block.data.scalar(), 2.0);
        p_block.run(dt, &BlockData::from_scalar(-2.0), None);
        assert_eq!(p_block.data.scalar(), -4.0);

        // Test integrator
        let mut i_block = PidBlock::new("PID2", &ic, 0.0, 3.0, 0.0, 10.0, 2.0);
        i_block.run(dt, &BlockData::from_scalar(0.0), None);
        i_block.run(dt, &BlockData::from_scalar(1.0), None);
        // Rectangular integration of error 1 and ki=3.0 for 1s should be 3.0
        assert_relative_eq!(i_block.data.scalar(), 3.0, max_relative = 0.01);

        // Should saturate at the i_max limit
        i_block.run(dt, &BlockData::from_scalar(100.0), None);
        assert_eq!(i_block.data.scalar(), 10.0);

        // Test derivative
        let mut d_block = PidBlock::new("PID3", &ic, 0.0, 0.0, 1.0, 0.0, 2.0);
        d_block.run(0.5, &BlockData::from_scalar(0.0), None); // Need at least 2 samples to estimate derivative
        d_block.run(0.5, &BlockData::from_scalar(100.0), None);
        assert_relative_eq!(d_block.data.scalar(), 200.0, max_relative = 0.01);

        // Test them all together!
        let mut pid_block = PidBlock::new("PID4", &ic, 1.0, 2.0, 3.0, 10.0, 2.0);
        pid_block.run(dt, &BlockData::from_scalar(0.0), None);
        pid_block.run(dt, &BlockData::from_scalar(2.0), None);
        // p = 2.0, i = 4.0, d = 6.0
        assert_relative_eq!(pid_block.data.scalar(), 12.0, max_relative = 0.01);
    }

    #[test]
    fn test_pid_block_vectorized() {
        let dt = 1.0;
        let ic = BlockData::from_vector(&[10., 0.]);

        let mut pid_block = PidBlock::new("PID5", &ic, 1.0, 2.0, 3.0, 30.0, 2.0);
        pid_block.run(dt, &BlockData::from_vector(&[0.0, 0.0]), None);
        pid_block.run(dt, &BlockData::from_vector(&[2.0, 4.0]), None);
        // p = (2.0, 4.0), i = (14.0, 8.0), d = (6.0, 12.0)
        assert_relative_eq!(
            pid_block.data,
            BlockData::from_vector(&[22.0, 24.0]),
            max_relative = 0.01
        );
    }
}
