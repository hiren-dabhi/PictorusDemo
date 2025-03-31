use core::time::Duration;
use corelib_traits::{DurationExt, GeneratorBlock, Scalar};
use num_traits::Float;
use utils::block_data::BlockData;

#[derive(Debug, Clone)]
pub struct SinewaveBlock<T>
where
    T: Scalar + Float,
    f64: From<T>,
{
    phantom: core::marker::PhantomData<T>,
    pub data: BlockData,
}

impl<T> Default for SinewaveBlock<T>
where
    T: Scalar + Float,
    f64: From<T>,
{
    fn default() -> Self {
        Self {
            phantom: core::marker::PhantomData,
            data: BlockData::from_scalar(f64::from(T::zero())),
        }
    }
}

impl<T> GeneratorBlock for SinewaveBlock<T>
where
    T: Scalar + Float,
    f64: From<T>,
    Duration: DurationExt<T>,
{
    type Parameters = Parameters<T>;
    type Output = T;

    fn generate(
        &mut self,
        parameters: &Self::Parameters,
        context: &dyn corelib_traits::Context,
    ) -> corelib_traits::PassBy<Self::Output> {
        let time = context.time().as_sec_float();
        let sin_val = parameters.amplitude
            * Float::sin(parameters.frequency * time + parameters.phase)
            + parameters.bias;
        self.data = BlockData::from_scalar(sin_val.into());
        sin_val
    }
}

#[derive(Debug, Clone)]
pub struct Parameters<T: Scalar> {
    pub amplitude: T,
    pub frequency: T,
    pub phase: T,
    pub bias: T,
}

impl<T: Scalar> Parameters<T> {
    pub fn new(amplitude: T, frequency: T, phase: T, bias: T) -> Self {
        Self {
            amplitude,
            frequency,
            phase,
            bias,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::time::Duration;
    use corelib_traits_testing::StubContext;

    #[test]
    fn test_sine_wave() {
        let mut block = SinewaveBlock::<f64>::default();
        let parameters = Parameters {
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.5,
            bias: 0.0,
        };

        let mut context = StubContext::new(Duration::from_secs(0), Duration::from_secs(0));

        assert_eq!(block.generate(&parameters, &context), Float::sin(0.5));
        assert_eq!(block.data.scalar(), Float::sin(0.5));
        context.time = Duration::from_secs(1);

        assert_eq!(block.generate(&parameters, &context), Float::sin(1.5));
        assert_eq!(block.data.scalar(), Float::sin(1.5));
    }
}
