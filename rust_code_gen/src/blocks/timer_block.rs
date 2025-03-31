use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum TimerEnum {
    CountDown,
    StopWatch,
}

pub struct TimerBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: TimerEnum,
    pub interruptable: bool,
    pub countdown_time_s: f64,
    pub timer_running: bool,
    pub start_time_s: f64,
}

impl TimerBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        method: &str,
        interruptable: bool,
        countdown_time_s: f64,
    ) -> TimerBlock {
        TimerBlock {
            name,
            method: method.parse().unwrap(),
            interruptable,
            countdown_time_s,
            data: ic.clone(),
            timer_running: false,
            start_time_s: 0.0,
        }
    }
    pub fn run(&mut self, time: f64, input_trigger: &BlockData) {
        let trigger_high = input_trigger.any();
        // Early exit if not running and input trigger is false
        if !self.timer_running && !trigger_high {
            debug!("{} data: {:?}", self.name, self.data);
            return;
        }

        if trigger_high {
            if !self.timer_running {
                // Start the timer
                self.start_time_s = time;
                self.timer_running = true;
            } else if self.timer_running && self.interruptable {
                // Interrupt and restart the timer
                self.start_time_s = time;
            }
        }

        let time_since_start = time - self.start_time_s;

        match self.method {
            TimerEnum::CountDown => {
                self._do_countdown(time_since_start);
            }
            TimerEnum::StopWatch => {
                self._do_stopwatch(time_since_start);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }

    fn _do_countdown(&mut self, time_since_start: f64) {
        if time_since_start < self.countdown_time_s {
            self.data
                .set_scalar(self.countdown_time_s - time_since_start);
        } else {
            self.data.set_scalar(0.0);
            self.timer_running = false;
        }
    }
    fn _do_stopwatch(&mut self, time_since_start: f64) {
        self.data.set_scalar(time_since_start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_timer_non_interruptable() {
        let ic = BlockData::from_scalar(0.0);
        let interruptable = false;
        let countdown_time_s = 5.0;
        let mut block = TimerBlock::new(
            "Countdown1",
            &ic,
            "CountDown",
            interruptable,
            countdown_time_s,
        );

        block.run(1.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(2.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 5.0);

        block.run(3.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 4.0);

        // Countdown not interrupted
        block.run(4.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 3.0);

        // Doesn't go below zero
        block.run(10.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(11.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);
    }

    #[test]
    fn test_countdown_timer_interruptable() {
        let ic = BlockData::from_scalar(0.0);
        let interruptable = true;
        let countdown_time_s = 5.0;
        let mut block = TimerBlock::new(
            "Countdown1",
            &ic,
            "CountDown",
            interruptable,
            countdown_time_s,
        );

        block.run(1.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(2.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 5.0);

        block.run(3.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 4.0);

        // Countdown interrupted
        block.run(4.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 5.0);

        // Doesn't go below zero
        block.run(5.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 5.0);

        block.run(6.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 4.0);
    }

    #[test]
    fn test_stopwatch_timer_non_interruptable() {
        let ic = BlockData::from_scalar(0.0);
        let interruptable = false;
        let countdown_time_s = -12.0; // Doesn't matter what this is for StopWatch
        let mut block = TimerBlock::new(
            "Countdown1",
            &ic,
            "StopWatch",
            interruptable,
            countdown_time_s,
        );

        block.run(1.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(2.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(3.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 1.0);

        // StopWatch not interrupted
        block.run(4.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 2.0);

        // Doesn't go below zero
        block.run(10.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 8.0);

        block.run(100.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 98.0);
    }

    #[test]
    fn test_stopwatch_timer_interruptable() {
        let ic = BlockData::from_scalar(0.0);
        let interruptable = true;
        let countdown_time_s = -12.0; // Doesn't matter what this is for StopWatch
        let mut block = TimerBlock::new(
            "Countdown1",
            &ic,
            "StopWatch",
            interruptable,
            countdown_time_s,
        );

        block.run(1.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(2.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(3.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 1.0);

        // StopWatch interrupted
        block.run(4.0, &BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 0.0);

        // Doesn't go below zero
        block.run(10.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 6.0);

        block.run(100.0, &BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 96.0);
    }
}
