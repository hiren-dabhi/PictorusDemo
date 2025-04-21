use corelib_traits::{HasIc, Pass, ProcessBlock};
use utils::{BlockData as OldBlockData, FromPass};

use crate::traits::CopyInto;

pub struct DelayBlock<T: Pass + Default + Copy, const N: usize>
where
    utils::BlockData: FromPass<T>,
{
    samples: [T; N],
    sample_index: usize,
    initial_accumulation: bool,
    output: T,
    pub data: OldBlockData,
}

impl<T: Pass + Default + Copy + CopyInto<T>, const N: usize> HasIc for DelayBlock<T, N>
where
    utils::BlockData: FromPass<T>,
{
    /// Constructs a new DelayBlock with the initial conditions from the parameters so that its output will be in a valid state before its first call to process.
    fn new(parameters: &Self::Parameters) -> Self {
        let mut output = Self::default();
        // Only setting the output and data fields here. After process has been called once the fields will be set with the IC on subsequent calls until N samples have been received.
        T::copy_into(parameters.ic.as_by(), &mut output.output);
        output.data = OldBlockData::from_pass(parameters.ic.as_by());
        output
    }
}

impl<T: Pass + Default + Copy, const N: usize> Default for DelayBlock<T, N>
where
    utils::BlockData: FromPass<T>,
{
    fn default() -> Self {
        Self {
            samples: [T::default(); N],
            initial_accumulation: true,
            output: T::default(),
            sample_index: 0,
            data: <OldBlockData as FromPass<T>>::from_pass(T::default().as_by()),
        }
    }
}

impl<T: Pass + Default + Copy + CopyInto<T>, const N: usize> ProcessBlock for DelayBlock<T, N>
where
    utils::BlockData: FromPass<T>,
{
    type Inputs = T;
    type Output = T;
    type Parameters = Parameters<T>;

    fn process<'b>(
        &'b mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: corelib_traits::PassBy<'_, Self::Inputs>,
    ) -> corelib_traits::PassBy<'b, Self::Output> {
        self.output = if self.initial_accumulation {
            parameters.ic
        } else {
            self.samples[self.sample_index]
        };

        // Store the current input in the sample buffer
        T::copy_into(inputs, &mut self.samples[self.sample_index]);

        // Increment the sample index, wrapping at N (and setting initial_accumulation to false)
        self.sample_index += 1;
        if self.sample_index >= N {
            self.initial_accumulation = false;
            self.sample_index = 0;
        }

        self.data = OldBlockData::from_pass(self.output.as_by());
        self.output.as_by()
    }
}

pub struct Parameters<T: Pass + Default + Copy> {
    ic: T,
}

impl<T: Pass + Default + Copy + CopyInto<T>> Parameters<T> {
    pub fn new(ic: T) -> Self {
        Self { ic }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corelib_traits::Matrix;
    use corelib_traits_testing::StubContext;

    #[test]
    fn test_delay_block_scalar() {
        let mut block = DelayBlock::<f64, 3>::default();
        let parameters = Parameters { ic: 0.0 };
        let context = StubContext::default();

        // Initial condition should be output until N samples are received
        assert_eq!(block.process(&parameters, &context, 1.0), 0.0);
        assert_eq!(block.process(&parameters, &context, 2.0), 0.0);
        assert_eq!(block.process(&parameters, &context, 3.0), 0.0);
        assert_eq!(block.process(&parameters, &context, 4.0), 1.0);
        assert_eq!(block.process(&parameters, &context, 5.0), 2.0);
        assert_eq!(block.process(&parameters, &context, 6.0), 3.0);
        assert_eq!(block.process(&parameters, &context, 7.0), 4.0);
        assert_eq!(block.process(&parameters, &context, 8.0), 5.0);
        assert_eq!(block.process(&parameters, &context, 9.0), 6.0);
        assert_eq!(block.process(&parameters, &context, 10.0), 7.0);
        assert_eq!(block.process(&parameters, &context, 11.0), 8.0);
        assert_eq!(block.process(&parameters, &context, 12.0), 9.0);
        assert_eq!(block.process(&parameters, &context, 13.0), 10.0);
        assert_eq!(block.process(&parameters, &context, 14.0), 11.0);
        assert_eq!(block.process(&parameters, &context, 15.0), 12.0);
        assert_eq!(block.process(&parameters, &context, 16.0), 13.0);
        assert_eq!(block.process(&parameters, &context, 17.0), 14.0);
        assert_eq!(block.process(&parameters, &context, 18.0), 15.0);
        assert_eq!(block.process(&parameters, &context, 19.0), 16.0);
        assert_eq!(block.process(&parameters, &context, 20.0), 17.0);
    }

    #[test]
    fn test_delay_block_matrix() {
        let mut block = DelayBlock::<Matrix<2, 2, f64>, 3>::default();
        let parameters = Parameters {
            ic: Matrix {
                data: [[0.0, 0.0], [0.0, 0.0]],
            },
        };
        let context = StubContext::default();

        // Initial condition should be output until N samples are received
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[1.0, 2.0], [3.0, 4.0]]
                }
            ),
            &Matrix {
                data: [[0.0, 0.0], [0.0, 0.0]]
            }
        );
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[5.0, 6.0], [7.0, 8.0]]
                }
            ),
            &Matrix {
                data: [[0.0, 0.0], [0.0, 0.0]]
            }
        );
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[9.0, 10.0], [11.0, 12.0]]
                }
            ),
            &Matrix {
                data: [[0.0, 0.0], [0.0, 0.0]]
            }
        );
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[13.0, 14.0], [15.0, 16.0]]
                }
            ),
            &Matrix {
                data: [[1.0, 2.0], [3.0, 4.0]]
            }
        );
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[17.0, 18.0], [19.0, 20.0]]
                }
            ),
            &Matrix {
                data: [[5.0, 6.0], [7.0, 8.0]]
            }
        );
        assert_eq!(
            block.process(
                &parameters,
                &context,
                &Matrix {
                    data: [[21.0, 22.0], [23.0, 24.0]]
                }
            ),
            &Matrix {
                data: [[9.0, 10.0], [11.0, 12.0]]
            }
        );
    }

    #[test]
    fn test_delay_block_scalar_ics() {
        let mut block = DelayBlock::<f64, 6>::default();
        let parameters = Parameters { ic: 42.0 };
        let context = StubContext::default();

        // Initial condition should be output until N samples are received
        assert_eq!(block.process(&parameters, &context, 1.0), 42.0);
        assert_eq!(block.process(&parameters, &context, 2.0), 42.0);
        assert_eq!(block.process(&parameters, &context, 3.0), 42.0);

        //switch it up parameter has a different IC now
        let parameters = Parameters { ic: 12.0 };
        assert_eq!(block.process(&parameters, &context, 4.0), 12.0);
        assert_eq!(block.process(&parameters, &context, 5.0), 12.0);
        assert_eq!(block.process(&parameters, &context, 6.0), 12.0);
        assert_eq!(block.process(&parameters, &context, 7.0), 1.0);
        assert_eq!(block.process(&parameters, &context, 8.0), 2.0);
        assert_eq!(block.process(&parameters, &context, 9.0), 3.0);
        assert_eq!(block.process(&parameters, &context, 10.0), 4.0);
        assert_eq!(block.process(&parameters, &context, 11.0), 5.0);
    }
}
