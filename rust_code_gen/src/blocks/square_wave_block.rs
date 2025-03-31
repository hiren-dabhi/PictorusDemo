use log::debug;

use crate::block_data::BlockData;

pub struct SquarewaveBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub amplitude: f64,
    pub on_duration: f64,
    pub off_duration: f64,
    pub bias: f64,
    pub phase: f64,
}

impl SquarewaveBlock {
    pub fn new(
        name: &'static str,
        amplitude: f64,
        on_duration: f64,
        off_duration: f64,
        bias: f64,
        phase: f64,
    ) -> SquarewaveBlock {
        SquarewaveBlock {
            name,
            data: BlockData::from_scalar(SquarewaveBlock::_data(
                0.0,
                bias,
                amplitude,
                on_duration,
                off_duration,
                phase,
            )),
            amplitude,
            on_duration,
            off_duration,
            bias,
            phase,
        }
    }
    pub fn run(&mut self, time: f64) {
        self.data.set_scalar(SquarewaveBlock::_data(
            time,
            self.bias,
            self.amplitude,
            self.on_duration,
            self.off_duration,
            self.phase,
        ));
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn _data(
        time: f64,
        bias: f64,
        amplitude: f64,
        on_duration: f64,
        off_duration: f64,
        phase: f64,
    ) -> f64 {
        let adjusted_time = time - phase;
        let time_since_last_pulse_start = adjusted_time % (on_duration + off_duration);
        if time_since_last_pulse_start < 0.0 {
            // This handles negative phases where the adjusted time could become negative.
            return bias;
        }
        if time_since_last_pulse_start <= on_duration {
            bias + amplitude
        } else {
            bias
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squarewave_block() {
        let amplitude = 2.0;
        let on_duration = 1.0;
        let off_duration = 2.0;
        let bias = 0.5;
        let phase = 0.5;

        let mut block = SquarewaveBlock::new(
            "Squarewave1",
            amplitude,
            on_duration,
            off_duration,
            bias,
            phase,
        );

        assert_eq!(block.data.scalar(), bias);

        block.run(0.5);
        assert_eq!(block.data.scalar(), bias + amplitude);

        block.run(1.0);
        assert_eq!(block.data.scalar(), bias + amplitude);

        block.run(1.499);
        assert_eq!(block.data.scalar(), bias + amplitude);

        block.run(1.5);
        assert_eq!(block.data.scalar(), bias + amplitude);

        // Off duration
        block.run(2.5);
        assert_eq!(block.data.scalar(), bias);

        block.run(3.4999);
        assert_eq!(block.data.scalar(), bias);

        // Back on
        block.run(3.5);
        assert_eq!(block.data.scalar(), bias + amplitude);
    }
}
