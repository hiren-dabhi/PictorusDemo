use core::str::FromStr;
use corelib_traits::{Matrix, Pass, PassBy, ProcessBlock, Scalar};
use pictorus_nalgebra_interop::MatrixExt;
use utils::{BlockData as OldBlockData, FromPass, ParseEnumError};
pub struct AggregateBlock<T: Apply> {
    pub data: OldBlockData,
    buffer: Option<T::Output>,
}

impl<T: Apply> Default for AggregateBlock<T> {
    fn default() -> Self {
        Self {
            data: OldBlockData::from_scalar(0.0),
            buffer: None,
        }
    }
}

impl<T> ProcessBlock for AggregateBlock<T>
where
    T: Apply,
    OldBlockData: FromPass<T::Output>,
{
    type Inputs = T;
    type Output = T::Output;
    type Parameters = AggregateMethod;

    fn process<'b>(
        &'b mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: corelib_traits::PassBy<'_, Self::Inputs>,
    ) -> corelib_traits::PassBy<'b, Self::Output> {
        let output = T::apply(&mut self.buffer, inputs, *parameters);
        self.data = OldBlockData::from_pass(output);
        output
    }
}

pub trait Apply: Pass {
    type Output: Scalar;

    fn apply<'s>(
        store: &mut Option<Self::Output>,
        input: PassBy<Self>,
        method: AggregateMethod,
    ) -> PassBy<'s, Self::Output>;
}

macro_rules! scalar_impls {
    () => {};
    ($type:ty, $($rest:tt),+) => {
        scalar_impls!($type);
        scalar_impls!($($rest),+);
    };
    ($type:ty) => {
        impl Apply for $type {
            type Output = $type;

            fn apply<'s>(
                store: &mut Option<Self::Output>,
                input: PassBy<Self>,
                _method: AggregateMethod,
            ) -> PassBy<'s, Self::Output> {
                *store = Some(input);
                input
            }
        }
    };
}
scalar_impls!(f64, f32); // We could also just easily add u8, u16 and bool here but they wouldn't have equivalent matrix impls

macro_rules! float_matrix_impl {
    ($type:ty) => {
        impl<const NROWS: usize, const NCOLS: usize> Apply for Matrix<NROWS, NCOLS, $type> {
            type Output = $type;

            fn apply<'s>(
                store: &mut Option<Self::Output>,
                input: PassBy<Self>,
                method: AggregateMethod,
            ) -> PassBy<'s, Self::Output> {
                let view = input.as_view();
                let output = match method {
                    AggregateMethod::Sum => view.sum(),
                    AggregateMethod::Mean => view.mean(),
                    AggregateMethod::Median => {
                        // Have to copy the data to the stack so we can sort it
                        let mut data = *input;
                        let data = data.data.as_flattened_mut();
                        view.iter().enumerate().for_each(|(i, &x)| data[i] = x);
                        data.sort_by(|a, b| a.partial_cmp(b).expect("NaNs are not supported"));
                        let mid = data.len() / 2;
                        if data.len() % 2 == 0 {
                            (data[mid - 1] + data[mid]) / Self::Output::from(2u8)
                        } else {
                            data[mid]
                        }
                    }
                    AggregateMethod::Min => view.min(),
                    AggregateMethod::Max => view.max(),
                };
                *store = Some(output);
                output
            }
        }
    };
}

float_matrix_impl!(f64);
float_matrix_impl!(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AggregateMethod {
    Sum,
    Mean,
    Median,
    Min,
    Max,
}

impl AggregateMethod {
    pub fn new(method: &str) -> Self {
        method
            .parse::<AggregateMethod>()
            .expect("Codgen Error, this should never fail")
    }
}

impl FromStr for AggregateMethod {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sum" => Ok(Self::Sum),
            "Mean" => Ok(Self::Mean),
            "Median" => Ok(Self::Median),
            "Min" => Ok(Self::Min),
            "Max" => Ok(Self::Max),
            _ => Err(ParseEnumError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use corelib_traits_testing::StubContext;

    #[test]
    fn test_aggregate_sum_f32() {
        let mut block = AggregateBlock::<Matrix<4, 7, f32>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Sum;
        let input: Matrix<4, 7, f32> = Matrix {
            data: [[1.0; 4]; 7],
        };
        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 28.0);
        assert_relative_eq!(block.data.scalar(), 28.0);
    }

    #[test]
    fn test_aggregate_sum_f64() {
        let mut block = AggregateBlock::<Matrix<4, 7, f64>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Sum;
        let input: Matrix<4, 7, f64> = Matrix {
            data: [[1.0; 4]; 7],
        };
        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 28.0);
        assert_relative_eq!(block.data.scalar(), 28.0);
    }

    #[test]
    fn test_aggregate_max_f64() {
        let mut block = AggregateBlock::<Matrix<4, 7, f64>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Max;
        let mut input: Matrix<4, 7, f64> = Matrix {
            data: [[1.0; 4]; 7],
        };
        input.data[5][3] = 42.0;
        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 42.0);
        assert_relative_eq!(block.data.scalar(), 42.0);
    }

    #[test]
    fn test_aggregate_min_f64() {
        let mut block = AggregateBlock::<Matrix<4, 7, f64>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Min;
        let mut input: Matrix<4, 7, f64> = Matrix {
            data: [[11.0; 4]; 7],
        };
        input.data[1][2] = 10.99;
        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 10.99);
        assert_relative_eq!(block.data.scalar(), 10.99);
    }

    #[test]
    fn test_aggregate_mean_f64() {
        let mut block = AggregateBlock::<Matrix<4, 7, f64>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Mean;
        let mut input: Matrix<4, 7, f64> = Matrix::zeroed();
        for (idx, elem) in input.data.as_flattened_mut().iter_mut().enumerate() {
            *elem = idx as f64;
        }

        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 13.5);
        assert_relative_eq!(block.data.scalar(), 13.5);
    }

    #[test]
    fn test_aggregate_median_f64() {
        let mut block = AggregateBlock::<Matrix<4, 7, f64>>::default();
        let context = StubContext::default();
        let params = AggregateMethod::Median;
        let mut input: Matrix<4, 7, f64> = Matrix::zeroed();
        for (idx, elem) in input.data.as_flattened_mut().iter_mut().enumerate() {
            *elem = idx as f64;
        }

        let output = block.process(&params, &context, &input);
        assert_relative_eq!(output, 13.5);
        assert_relative_eq!(block.data.scalar(), 13.5);
    }

    #[test]
    fn test_aggregate_method_from_str() {
        assert_eq!(
            AggregateMethod::from_str("Sum").unwrap(),
            AggregateMethod::Sum
        );
        assert_eq!(
            AggregateMethod::from_str("Mean").unwrap(),
            AggregateMethod::Mean
        );
        assert_eq!(
            AggregateMethod::from_str("Median").unwrap(),
            AggregateMethod::Median
        );
        assert_eq!(
            AggregateMethod::from_str("Min").unwrap(),
            AggregateMethod::Min
        );
        assert_eq!(
            AggregateMethod::from_str("Max").unwrap(),
            AggregateMethod::Max
        );
        assert!(AggregateMethod::from_str("Invalid").is_err());
    }
}
