use crate::block_data::BlockData;
use log::debug;
use num_traits::Float;

pub struct SinewaveBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub amplitude: f64,
    pub frequency: f64,
    pub phase: f64,
    pub bias: f64,
}

impl SinewaveBlock {
    pub fn new(
        name: &'static str,
        amplitude: f64,
        frequency: f64,
        phase: f64,
        bias: f64,
    ) -> SinewaveBlock {
        SinewaveBlock {
            name,
            data: BlockData::from_scalar(SinewaveBlock::_data(
                0.0, amplitude, frequency, phase, bias,
            )),
            amplitude,
            frequency,
            phase,
            bias,
        }
    }
    pub fn run(&mut self, time: f64) {
        self.data.set_scalar(SinewaveBlock::_data(
            time,
            self.amplitude,
            self.frequency,
            self.phase,
            self.bias,
        ));
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn _data(time: f64, amplitude: f64, frequency: f64, phase: f64, bias: f64) -> f64 {
        amplitude * Float::sin(frequency * time + phase) + bias
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinewave_block() {
        let amplitude = 1.0;
        let frequency = 1.0;
        let phase = 0.5;
        let bias = 0.0;

        let mut block = SinewaveBlock::new("Sinewave1", amplitude, frequency, phase, bias);

        // Should initially output sine of phase
        assert_eq!(block.data.scalar(), Float::sin(0.5_f64));

        block.run(1.0);
        assert_eq!(block.data.scalar(), Float::sin(1.5_f64));
    }
}
