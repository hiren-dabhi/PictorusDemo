use crate::block_data::BlockData;
use log::debug;

pub struct IirFilterBlock {
    pub name: &'static str,
    pub time_constant_s: f64,
    pub data: BlockData,
}

impl IirFilterBlock {
    pub fn new(name: &'static str, ic: &BlockData, time_constant_s: f64) -> IirFilterBlock {
        IirFilterBlock {
            name,
            time_constant_s,
            data: ic.clone(),
        }
    }

    pub fn run(&mut self, timestep_s: f64, sample: &BlockData) {
        let alpha = timestep_s / (timestep_s + self.time_constant_s);
        self.data = alpha * sample + &((1.0 - alpha) * &self.data);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_iir_filter_block() {
        // Use 1s settling time
        let time_constants_s = 1.0;
        let mut block = IirFilterBlock::new(
            "IirFilter1",
            &BlockData::from_vector(&[0.0, 0.0]),
            time_constants_s,
        );

        // Sending in unity with 1s timestamps should result in filter reaching
        // roughly 50% of final data
        block.run(1.0, &BlockData::from_vector(&[1.0, 2.0]));
        assert_relative_eq!(
            block.data,
            BlockData::from_vector(&[0.5, 1.0]),
            max_relative = 0.01
        );
    }
}
