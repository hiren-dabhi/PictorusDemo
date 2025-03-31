use log::debug;

use crate::block_data::BlockData;

pub struct RateLimitBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub rising_rate: BlockData,
    pub falling_rate: BlockData,
}

impl RateLimitBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        rising_rate: f64,
        falling_rate: f64,
    ) -> RateLimitBlock {
        RateLimitBlock {
            name,
            data: ic.clone(),
            rising_rate: BlockData::scalar_sizeof(rising_rate, ic),
            falling_rate: BlockData::scalar_sizeof(falling_rate, ic),
        }
    }
    pub fn run(&mut self, input: &BlockData, timestep_s: f64) {
        if timestep_s <= f64::EPSILON {
            debug!("Timestep is zero, not computing rate of change!");
            return;
        }
        let mut change_rate = (input - &self.data) / timestep_s;
        let apply_rising_rate = change_rate.gt(&self.rising_rate);
        let apply_falling_rate = change_rate.lt(&self.falling_rate);

        change_rate.component_set(&apply_rising_rate, &self.rising_rate);
        change_rate.component_set(&apply_falling_rate, &self.falling_rate);

        self.data += &(change_rate * timestep_s);

        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_block() {
        let rising_rate: f64 = 2.0;
        let falling_rate: f64 = -1.0;
        let ic = BlockData::from_scalar(0.0);
        let mut block = RateLimitBlock::new("RateLimit1", &ic, rising_rate, falling_rate);

        let timestep_s = 1.0;

        // Test rising rate
        block.run(&BlockData::from_scalar(3.0), timestep_s);
        assert_eq!(block.data.scalar(), 2.0);
        block.run(&BlockData::from_scalar(30.0), timestep_s);
        assert_eq!(block.data.scalar(), 4.0);

        // Value doesn't change if input matches current state
        block.run(&BlockData::from_scalar(4.0), timestep_s);
        assert_eq!(block.data.scalar(), 4.0);

        // Test falling rate
        block.run(&BlockData::from_scalar(-30.0), timestep_s);
        assert_eq!(block.data.scalar(), 3.0);
        block.run(&BlockData::from_scalar(-0.5), timestep_s);
        assert_eq!(block.data.scalar(), 2.0);

        // Test passing in no timestep does not change output
        block.run(&BlockData::from_scalar(-30.0), 0.0);
        assert_eq!(block.data.scalar(), 2.0);
    }
}
