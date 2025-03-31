use crate::block_data::BlockData;
use alloc::collections::VecDeque;
use log::debug;

#[derive(strum::EnumString)]
pub enum DelayEnum {
    Iterations,
    Time,
}

pub struct DelayBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub delay_value: f64,
    pub last_publish_time: f64,
    pub delayed_inputs: VecDeque<BlockData>,
    pub method: DelayEnum,
}

impl DelayBlock {
    pub fn new(name: &'static str, ic: &BlockData, delay_value: f64, method: &str) -> DelayBlock {
        DelayBlock {
            name,
            data: ic.clone(),
            delay_value,
            delayed_inputs: VecDeque::with_capacity(delay_value as usize),
            last_publish_time: 0.0,
            method: method
                .parse()
                .expect("Expected 'method' to be a string: 'Iterations' | 'Time'"),
        }
    }
    pub fn run(&mut self, input: &BlockData, app_time_s: f64) {
        match self.method {
            DelayEnum::Iterations => self.run_iterations_method(input),
            DelayEnum::Time => self.run_time_method(input, app_time_s),
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn run_iterations_method(&mut self, input: &BlockData) {
        self.delayed_inputs.push_back(input.clone());
        if self.delayed_inputs.len() <= self.delay_value as usize {
            return;
        }
        self.data = self.delayed_inputs.front().unwrap().clone();
        self.delayed_inputs.pop_front();
    }
    fn run_time_method(&mut self, input: &BlockData, app_time_s: f64) {
        if self.delay_value == 0.0 {
            self.data = input.clone();
            self.delayed_inputs.clear(); // Clear the queue when delay is 0
            return;
        }

        self.delayed_inputs.push_back(input.clone());

        if app_time_s < self.delay_value {
            self.data = self.data.clone(); // Use the initial condition
        } else {
            self.data = self.delayed_inputs.front().unwrap().clone();
            self.delayed_inputs.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_block_by_iterations() {
        let time = 0.0;
        let mut delay_block =
            DelayBlock::new("Delay1", &BlockData::from_scalar(0.0), 3.0, "Iterations");

        // Outputs IC until N delays
        delay_block.run(&BlockData::from_scalar(1.0), time);
        assert_eq!(delay_block.data.scalar(), 0.0);

        delay_block.run(&BlockData::from_scalar(2.0), time);
        assert_eq!(delay_block.data.scalar(), 0.0);

        delay_block.run(&BlockData::from_scalar(3.0), time);
        assert_eq!(delay_block.data.scalar(), 0.0);

        delay_block.run(&BlockData::from_scalar(4.0), time);
        assert_eq!(delay_block.data.scalar(), 1.0);

        delay_block.run(&BlockData::from_scalar(5.0), time);
        assert_eq!(delay_block.data.scalar(), 2.0);
    }

    #[test]
    fn test_delay_block_by_time() {
        let mut delay_block = DelayBlock::new("Delay1", &BlockData::from_scalar(0.0), 2.0, "Time");

        // Outputs IC until 2s surpass
        delay_block.run(&BlockData::from_scalar(1.0), 0.0);
        assert_eq!(delay_block.data.scalar(), 0.0);

        delay_block.run(&BlockData::from_scalar(2.0), 1.0);
        assert_eq!(delay_block.data.scalar(), 0.0);

        delay_block.run(&BlockData::from_scalar(3.0), 2.0);
        assert_eq!(delay_block.data.scalar(), 1.0);

        // Hold last value, not enough time has passed
        delay_block.run(&BlockData::from_scalar(4.0), 3.0);
        assert_eq!(delay_block.data.scalar(), 2.0);

        delay_block.run(&BlockData::from_scalar(5.0), 4.0);
        assert_eq!(delay_block.data.scalar(), 3.0);

        // Emit 4, skipping 3
        delay_block.run(&BlockData::from_scalar(6.0), 5.0);
        assert_eq!(delay_block.data.scalar(), 4.0);

        delay_block.run(&BlockData::from_scalar(6.0), 6.0);
        assert_eq!(delay_block.data.scalar(), 5.0);
    }
}
