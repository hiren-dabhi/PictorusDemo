use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use crate::block_data::{BlockData, BlockDataType};
use log::debug;
use nalgebra::DMatrix;
use rustfft::{num_complex::Complex, Fft, FftPlanner};

pub struct FFTBlock {
    pub name: &'static str,
    buffer: Vec<Complex<f64>>,
    fft: Arc<dyn Fft<f64>>,
    buffer_size: usize,
    sample_count: usize,
    pub data: BlockData,
}

impl FFTBlock {
    pub fn new(name: &'static str, ic: &BlockData, buffer_size: f64) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(buffer_size as usize);

        FFTBlock {
            name,
            buffer: vec![Complex { re: 0.0, im: 0.0 }; buffer_size as usize],
            buffer_size: buffer_size as usize,
            fft,
            sample_count: 0,
            data: ic.clone(),
        }
    }

    pub fn run(&mut self, input_sample: &BlockData) {
        let vector = input_sample.get_data();
        for i in 0..vector.len() {
            let value = vector[i];

            // Assuming 0.0 is not a valid value and indicates the end of valid data
            // This would indicate the buffer allocated to input_sample was not completely
            // populated with sensor data last timestep. (TODO: Is this better than f64::MIN?)
            if value == 0.0 {
                break;
            }

            // Add sample to buffer, if buffer not full
            if self.sample_count == self.buffer_size {
                break;
            }

            self.buffer[self.sample_count] = Complex { re: value, im: 0.0 };
            self.sample_count += 1;
        }

        // Check if buffer is full
        if self.sample_count == self.buffer_size {
            // Perform FFT
            self.fft.process(&mut self.buffer);

            /*   --- Process FFT output, which is stored in the buffer ----
            Output depends on sampling rate (Fs), so the frequency represented by the i-th index
            in your FFT result is i * Fs / buffer_size. If the sampling rate is not specified,
            it's crucial to know it for accurate frequency analysis.

            Example:
            Let's say your sampling rate Fs is 100 Hz, and your buffer size N is 5. The frequency for each bin would be:

            Bin 0: 0 * 100 / 5 = 0 Hz (DC component - the average signal strength)
            Bin 1: 1 * 100 / 5 = 20 Hz
            Bin 2: 2 * 100 / 5 = 40 Hz
            ... and so on.
            */
            let mut new_data = DMatrix::zeros(2, self.buffer_size);
            for (i, complex) in self.buffer.iter().enumerate() {
                new_data[(0, i)] = complex.re; // Store real part
                new_data[(1, i)] = complex.im; // Store imaginary part
            }
            self.data = BlockData::from_data(new_data, BlockDataType::Matrix);

            // Reset sample count for next batch
            self.sample_count = 0;
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;
    use crate::blocks::sinewave_block::SinewaveBlock;
    use approx::assert_relative_eq;

    #[test]
    fn test_fft_block() {
        let mut fft_block = FFTBlock::new("fft_block", &BlockData::from_element(2, 10, 0.0), 10.0);

        // 2Hz sinewave, amplitude 5, with small bias
        let mut sinewave_2hz = SinewaveBlock::new("2Hz", 5., 2. * 6.283, 0., 1.2345);
        // 3Hz sinewave, amplitude 10, with small phase shift
        let mut sinewave_3hz = SinewaveBlock::new("3Hz", 10., 3. * 6.283, 0.5, 0.0);

        // Simulate a 10Hz sampling rate
        for i in 0..100 {
            let time = i as f64 * 0.10; // Increment time by 0.10 seconds for each step

            sinewave_2hz.run(time);
            sinewave_3hz.run(time);

            let combined =
                BlockData::from_scalar(sinewave_2hz.data.scalar() + sinewave_3hz.data.scalar());

            fft_block.run(&combined);
        }

        let fft_magnitudes = fft_block.data.vector_magnitude();

        let expected = BlockData::from_row_slice(
            1,
            10,
            &[
                12.3474, // DC value (bias in the signal from 2hz signal)
                0.00327, // 1Hz
                24.9963, // 2Hz response twice as strong as DC bias
                50.0026, // 3Hz response twice as strong as 2Hz response
                0.00677, // 4Hz
                0.00518, // 5Hz
                0.00677, // 6Hz
                50.0026, // 7Hz - higher order harmonics
                24.9963, // 8Hz - higher order harmonics
                0.00327, // 9Hz
            ],
        );

        assert_relative_eq!(fft_magnitudes, expected, max_relative = 0.01);
    }
}
