use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
enum DelayControlMethod {
    Debounce,
    Throttle,
}

pub struct DelayControlBlock {
    pub name: &'static str,
    pub data: BlockData,
    method: DelayControlMethod,
    pub delay_time_s: f64,
    control_time: f64,
    ready_to_emit: bool,
}

impl DelayControlBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        delay_time_s: f64,
        method: &str,
    ) -> DelayControlBlock {
        DelayControlBlock {
            name,
            data: ic.clone(),
            method: method.parse().unwrap(),
            delay_time_s,
            control_time: -delay_time_s,
            ready_to_emit: false,
        }
    }
    pub fn run(&mut self, input: &BlockData, app_time: f64) {
        let input_is_true = input.any();
        match self.method {
            DelayControlMethod::Debounce => self.debounce(input_is_true, app_time),
            DelayControlMethod::Throttle => self.throttle(input_is_true, app_time),
        };
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn debounce(&mut self, input_is_true: bool, time: f64) {
        let mut new_val = 0.0;
        if input_is_true {
            self.control_time = time;
            self.ready_to_emit = true;
        } else if self.ready_to_emit && time >= self.control_time + self.delay_time_s {
            new_val = 1.0;
            self.ready_to_emit = false;
        }
        self.data.set_scalar(new_val);
    }
    fn throttle(&mut self, input_is_true: bool, time: f64) {
        let mut new_val = 0.0;
        if input_is_true && time >= self.control_time + self.delay_time_s {
            new_val = 1.0;
            self.control_time = time;
        }
        self.data.set_scalar(new_val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_control_block_debounce() {
        let ic = BlockData::from_scalar(0.0);
        let delay_time = 1.2;
        let mut block = DelayControlBlock::new("DelayControl", &ic, delay_time, "Debounce");

        let true_input: BlockData = BlockData::from_scalar(1.0);
        let false_input: BlockData = BlockData::from_scalar(0.0);

        // No true input yet...
        block.run(&false_input, 0.0);
        assert_eq!(block.data.scalar(), 0.0);

        // 1s later, we get true input (still no emit)
        block.run(&true_input, 1.0);
        assert_eq!(block.data.scalar(), 0.0);

        // 1s later, we get false input (still no emit)
        block.run(&false_input, 2.0);
        assert_eq!(block.data.scalar(), 0.0);

        // 0.3s later, still false input (should emit)
        block.run(&false_input, 2.3);
        assert_eq!(block.data.scalar(), 1.0);

        // Next time step should be back to false, even with true input
        block.run(&true_input, 2.31);
        assert_eq!(block.data.scalar(), 0.0);

        // 1.0s later, true input reset debounce
        block.run(&true_input, 3.31);
        assert_eq!(block.data.scalar(), 0.0);

        // 1.19s later, still false output
        block.run(&false_input, 4.5);
        assert_eq!(block.data.scalar(), 0.0);

        // Next timestep we should emit true
        block.run(&false_input, 4.51);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_delay_control_block_throttle() {
        let ic = BlockData::from_scalar(0.0);
        let delay_time = 1.2;
        let mut block = DelayControlBlock::new("DelayControl", &ic, delay_time, "Throttle");

        let true_input: BlockData = BlockData::from_scalar(1.0);
        let false_input: BlockData = BlockData::from_scalar(0.0);

        // No true input yet...
        block.run(&false_input, 0.0);
        assert_eq!(block.data.scalar(), 0.0);

        // 1s later, we get true input (immediately emit)
        block.run(&true_input, 1.0);
        assert_eq!(block.data.scalar(), 1.0);

        // 0.5s later, false input (emit false)
        block.run(&true_input, 1.5);
        assert_eq!(block.data.scalar(), 0.0);

        // 0.5s later, true input (emit false, still cooling down)
        block.run(&true_input, 2.0);
        assert_eq!(block.data.scalar(), 0.0);

        // 0.2s later, emits true (cooldown over)
        block.run(&true_input, 2.2);
        assert_eq!(block.data.scalar(), 1.0);

        // Next timestep, emits false
        block.run(&true_input, 2.21);
        assert_eq!(block.data.scalar(), 0.0);
    }
}
