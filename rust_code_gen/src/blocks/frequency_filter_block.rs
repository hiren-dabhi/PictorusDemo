use crate::block_data::BlockData;
use core::f64::consts::PI;
use log::debug;

#[derive(strum::Display, strum::EnumString)]
pub enum FrequencyFilterEnum {
    HighPass,
    LowPass,
}

pub struct FrequencyFilterBlock {
    pub name: &'static str,
    pub method: FrequencyFilterEnum,
    pub cutoff_frequency: f64,
    prev_input: BlockData,
    prev_output: BlockData,
    prev_time: f64,
    pub data: BlockData,
}

impl FrequencyFilterBlock {
    pub fn new(name: &'static str, ic: &BlockData, cutoff_frequency: f64, method: &str) -> Self {
        FrequencyFilterBlock {
            name,
            method: method.parse().unwrap(),
            cutoff_frequency,
            prev_input: BlockData::scalar_sizeof(0.0, ic),
            prev_output: ic.clone(),
            prev_time: 0.0,
            data: ic.clone(),
        }
    }

    pub fn run(&mut self, app_time_s: f64, input_sample: &BlockData) {
        let timestep_s = app_time_s - self.prev_time;
        let alpha = {
            let alpha = match self.method {
                FrequencyFilterEnum::HighPass => {
                    1.0 / (1.0 + (2.0 * PI * self.cutoff_frequency * timestep_s))
                }
                FrequencyFilterEnum::LowPass => {
                    (2.0 * PI * self.cutoff_frequency * timestep_s)
                        / (1.0 + (2.0 * PI * self.cutoff_frequency * timestep_s))
                }
            };
            BlockData::scalar_sizeof(alpha, input_sample)
        };

        self.data = match self.method {
            FrequencyFilterEnum::HighPass => {
                alpha.component_mul(&(&self.prev_output + input_sample - &self.prev_input))
            }
            FrequencyFilterEnum::LowPass => {
                &self.prev_output + &alpha.component_mul(&(input_sample - &self.prev_output))
            }
        };
        debug!("{} data: {:?}", self.name, self.data);

        self.prev_input = input_sample.clone();
        self.prev_output = self.data.clone();
        self.prev_time = app_time_s;
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::iter;

    use super::*;
    use crate::blocks::sinewave_block::SinewaveBlock;

    fn rms(data: &[f64]) -> f64 {
        let sum: f64 = data.iter().map(|x| x.powi(2)).sum();
        (sum / data.len() as f64).sqrt()
    }

    #[test]
    fn test_frequency_filter_block_high_pass() {
        let mut high_pass_50hz = FrequencyFilterBlock::new(
            "high_pass_50hz",
            &BlockData::from_scalar(0.0),
            50.0,
            "HighPass",
        );

        let mut high_pass_1hz = FrequencyFilterBlock::new(
            "high_pass_1hz",
            &BlockData::from_scalar(0.0),
            1.0,
            "HighPass",
        );

        let mut sine_25hz = SinewaveBlock::new("sinewave", 1.0, 25.0 * 2.0 * PI, 0., 0.);

        let mut sine_data = [0.0; 1000];
        let mut hp_50_data = [0.0; 1000];
        let mut hp_1_data = [0.0; 1000];

        for time in 0..1000 {
            let time_s = time as f64 * 0.001;
            sine_25hz.run(time_s);
            high_pass_50hz.run(time_s, &sine_25hz.data);
            high_pass_1hz.run(time_s, &sine_25hz.data);
            sine_data[time] = sine_25hz.data.scalar();
            hp_50_data[time] = high_pass_50hz.data.scalar();
            hp_1_data[time] = high_pass_1hz.data.scalar();
        }

        // Assert the the 1 HZ highpassed signal is within 5% of the original signal since the 25 Hz is well above the cutoff fequency
        assert!((rms(&sine_data) - rms(&hp_1_data)) / rms(&sine_data) < 0.05);
        //Assert that the 50Hz highpassed signal is within 50% of the original signal since the 25 Hz is below the cutoff frequency and it has started to roll off
        assert!(rms(&hp_50_data) < 0.5 * rms(&sine_data));
    }

    #[test]
    fn test_frequency_filter_block_low_pass() {
        let mut low_pass_50hz = FrequencyFilterBlock::new(
            "low_pass_50hz",
            &BlockData::from_scalar(0.0),
            50.0,
            "LowPass",
        );

        let mut low_pass_1hz =
            FrequencyFilterBlock::new("low_pass_1hz", &BlockData::from_scalar(0.0), 1.0, "LowPass");

        let mut sine_25hz = SinewaveBlock::new("sinewave", 1.0, 25.0 * 2.0 * PI, 0., 0.);

        let mut sine_data = [0.0; 1000];
        let mut lp_50_data = [0.0; 1000];
        let mut lp_1_data = [0.0; 1000];

        for time in 0..1000 {
            let time_s = time as f64 * 0.001;
            sine_25hz.run(time_s);
            low_pass_50hz.run(time_s, &sine_25hz.data);
            low_pass_1hz.run(time_s, &sine_25hz.data);
            sine_data[time] = sine_25hz.data.scalar();
            lp_50_data[time] = low_pass_50hz.data.scalar();
            lp_1_data[time] = low_pass_1hz.data.scalar();
        }

        // Assert the the 1 HZ lowpassed signal is less than 5% of the original signal since the 25 Hz is well above the cutoff fequency
        assert!(rms(&lp_1_data) < 0.05 * rms(&sine_data));
        //Assert that the 50Hz lowpassed signal is within 85% of the original signal since the 25 Hz is below the cutoff frequency
        assert!((rms(&sine_data) - rms(&lp_50_data)) / rms(&sine_data) < 0.15);
    }

    #[test]
    fn test_matrix_input() {
        // (2,3) matrix filter
        let mut high_pass_50hz_mat = FrequencyFilterBlock::new(
            "high_pass_50hz",
            &BlockData::from_matrix(&[&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]]),
            50.0,
            "HighPass",
        );

        // Six scalar filters
        let mut high_pass_50_hz_scalars: Vec<_> = iter::repeat_with(|| {
            FrequencyFilterBlock::new(
                "high_pass_50hz",
                &BlockData::from_scalar(0.0),
                50.0,
                "HighPass",
            )
        })
        .take(6)
        .collect();

        // Six sine generators from 10 to 60 Hz
        let mut sine_generators: Vec<_> = (1..7)
            .map(f64::from)
            .map(|x| 2. * PI * 10. * x)
            .map(|x| SinewaveBlock::new("sinewave", 1.0, x * 2.0 * PI, 0., 0.))
            .collect();

        for i in 0..1000 {
            let time_s = i as f64 * 0.001;
            sine_generators.iter_mut().for_each(|sine| sine.run(time_s));

            let sine_data_block: BlockData = BlockData::from_matrix(&[
                &[
                    sine_generators[0].data.scalar(),
                    sine_generators[1].data.scalar(),
                    sine_generators[2].data.scalar(),
                ],
                &[
                    sine_generators[3].data.scalar(),
                    sine_generators[4].data.scalar(),
                    sine_generators[5].data.scalar(),
                ],
            ]);
            high_pass_50hz_mat.run(time_s, &sine_data_block);

            for (filter, sine) in high_pass_50_hz_scalars
                .iter_mut()
                .zip(sine_generators.iter())
            {
                filter.run(time_s, &sine.data);
            }

            // Assert that the matrix filter is the same as the scalar filters
            assert_eq!(
                high_pass_50hz_mat.data,
                BlockData::from_matrix(&[
                    &[
                        high_pass_50_hz_scalars[0].data.scalar(),
                        high_pass_50_hz_scalars[1].data.scalar(),
                        high_pass_50_hz_scalars[2].data.scalar(),
                    ],
                    &[
                        high_pass_50_hz_scalars[3].data.scalar(),
                        high_pass_50_hz_scalars[4].data.scalar(),
                        high_pass_50_hz_scalars[5].data.scalar(),
                    ]
                ])
            );
        }
    }
}
