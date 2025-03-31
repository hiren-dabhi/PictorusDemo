use core::time::Duration;
use corelib_traits::{DurationExt, GeneratorBlock, Scalar};
use num_traits::Float;
use utils::block_data::BlockData;

#[derive(Debug, Clone)]
pub struct RampBlock<T: Scalar + Float> {
    phantom: core::marker::PhantomData<T>,
    pub data: BlockData,
}

impl<T: Scalar + Float> Default for RampBlock<T>
where
    f64: From<T>,
{
    fn default() -> Self {
        Self {
            phantom: core::marker::PhantomData,
            data: BlockData::from_scalar(f64::from(T::default())),
        }
    }
}

impl<T> GeneratorBlock for RampBlock<T>
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
        let ramp_val = parameters.rate * (time - parameters.start_time).max(T::zero());
        self.data = BlockData::from_scalar(ramp_val.into());
        ramp_val
    }
}

#[derive(Debug)]
pub struct Parameters<T: Scalar + Float> {
    pub start_time: T,
    pub rate: T,
}

impl<T: Scalar + Float> Parameters<T> {
    pub fn new(start_time: T, rate: T) -> Self {
        Self { start_time, rate }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::time::Duration;
    use corelib_traits_testing::{StubContext, StubRuntime};

    #[test]
    fn test_ramp_block() {
        let mut block = RampBlock::<f64>::default();
        let mut runtime = StubRuntime::new(StubContext::new(
            Duration::from_secs_f64(0.0),
            Duration::from_secs_f64(1.0),
        ));

        // Slope is 1.0, start time is 0.0
        let parameters = Parameters::new(0.0, 1.0);
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 0.0);

        runtime.tick();
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 1.0);

        runtime.tick();
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 2.0);

        // Slope is 3.0, start time is 1.0
        runtime.context.time = Duration::from_secs_f64(0.0); //reset time
        let parameters = Parameters::new(1.0, 3.0);
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 0.0);

        runtime.tick();
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 0.0);

        runtime.tick();
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 3.0);

        runtime.tick();
        let output = block.generate(&parameters, &runtime.context());
        assert_eq!(output, 6.0);
    }
}
