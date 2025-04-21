use crate::traits::{Apply, ApplyInto, MatrixOps, Scalar};
use core::str::FromStr;
use corelib_traits::{Matrix, Pass, PassBy, ProcessBlock};
use utils::{BlockData as OldBlockData, FromPass, ParseEnumError};

/// The type of comparison operation to perform
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

impl ComparisonType {
    pub fn new(method: &str) -> Self {
        method
            .parse::<ComparisonType>()
            .expect("Failed to parse comparison method.")
    }
}

impl FromStr for ComparisonType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Equal" => Ok(Self::Equal),
            "NotEqual" => Ok(Self::NotEqual),
            "GreaterThan" => Ok(Self::GreaterThan),
            "GreaterOrEqual" => Ok(Self::GreaterOrEqual),
            "LessThan" => Ok(Self::LessThan),
            "LessOrEqual" => Ok(Self::LessOrEqual),
            _ => Err(ParseEnumError),
        }
    }
}

/// Parameters for the comparison operator block
pub struct Parameters {
    pub comparison_type: ComparisonType,
}

impl Parameters {
    pub fn new(comparison_type: &str) -> Self {
        Self {
            comparison_type: ComparisonType::new(comparison_type),
        }
    }
}

/// Performs an element-wise comparison operation on two inputs.
/// Currently supports the following comparison methods:
/// - Equal
/// - NotEqual
/// - GreaterThan
/// - GreaterOrEqual
/// - LessThan
/// - LessOrEqual
pub struct ComparisonBlock<T>
where
    T: Apply<Parameters>,
    OldBlockData: FromPass<<T as Apply<Parameters>>::Output>,
{
    pub data: OldBlockData,
    buffer: Option<T::Output>,
}

impl<T> Default for ComparisonBlock<T>
where
    T: Apply<Parameters>,
    OldBlockData: FromPass<<T as Apply<Parameters>>::Output>,
{
    fn default() -> Self {
        Self {
            data: <OldBlockData as FromPass<T::Output>>::from_pass(T::Output::default().as_by()),
            buffer: None,
        }
    }
}

impl<T> ProcessBlock for ComparisonBlock<T>
where
    T: Apply<Parameters>,
    OldBlockData: FromPass<<T as Apply<Parameters>>::Output>,
{
    type Inputs = T;
    type Output = T::Output;
    type Parameters = Parameters;

    fn process<'b>(
        &'b mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: PassBy<Self::Inputs>,
    ) -> PassBy<'b, Self::Output> {
        self.buffer = None;
        T::apply(inputs, parameters, &mut self.buffer);
        self.data = OldBlockData::from_pass(self.buffer.as_ref().unwrap().as_by());
        self.buffer.as_ref().unwrap().as_by()
    }
}

fn perform_op<S: Scalar + core::cmp::PartialEq + core::cmp::PartialOrd + From<bool>>(
    lhs: S,
    rhs: S,
    comparison_type: ComparisonType,
) -> S {
    let res = match comparison_type {
        ComparisonType::Equal => rhs == lhs,
        ComparisonType::NotEqual => rhs != lhs,
        ComparisonType::GreaterThan => rhs > lhs,
        ComparisonType::GreaterOrEqual => rhs >= lhs,
        ComparisonType::LessThan => rhs < lhs,
        ComparisonType::LessOrEqual => rhs <= lhs,
    };
    res.into()
}

// Compare scalar with scalar
impl<S: Scalar + core::cmp::PartialEq + core::cmp::PartialOrd + From<bool>> ApplyInto<S, Parameters>
    for S
{
    fn apply_into<'a>(
        input: PassBy<Self>,
        params: &Parameters,
        dest: &'a mut Option<S>,
    ) -> PassBy<'a, S> {
        match dest {
            Some(dest) => {
                *dest = perform_op(input, *dest, params.comparison_type);
            }
            None => {
                *dest = Some(input);
            }
        }

        dest.as_ref().unwrap().as_by()
    }
}

// Compare matrix and matrix
impl<
        const R: usize,
        const C: usize,
        S: Scalar + core::cmp::PartialEq + core::cmp::PartialOrd + From<bool>,
    > ApplyInto<Matrix<R, C, S>, Parameters> for Matrix<R, C, S>
{
    fn apply_into<'a>(
        input: PassBy<Self>,
        params: &Parameters,
        dest: &'a mut Option<Matrix<R, C, S>>,
    ) -> PassBy<'a, Matrix<R, C, S>> {
        match dest {
            Some(dest) => {
                input
                    .data
                    .as_flattened()
                    .iter()
                    .zip(dest.data.as_flattened_mut().iter_mut())
                    .for_each(|(input, dest)| {
                        *dest = perform_op(*input, *dest, params.comparison_type);
                    });
            }
            None => {
                *dest = Some(*input);
            }
        }

        dest.as_ref().unwrap().as_by()
    }
}

// Compare scalar with matrix
impl<
        const R: usize,
        const C: usize,
        S: Scalar + core::cmp::PartialEq + core::cmp::PartialOrd + From<bool>,
    > ApplyInto<Matrix<R, C, S>, Parameters> for S
{
    fn apply_into<'a>(
        input: PassBy<Self>,
        params: &Parameters,
        dest: &'a mut Option<Matrix<R, C, S>>,
    ) -> PassBy<'a, Matrix<R, C, S>> {
        match dest {
            Some(dest) => {
                dest.data.as_flattened_mut().iter_mut().for_each(|dest| {
                    *dest = perform_op(input, *dest, params.comparison_type);
                });
            }
            None => {
                *dest = Some(Matrix::<R, C, S>::from_element(input));
            }
        }

        dest.as_ref().unwrap().as_by()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use corelib_traits_testing::StubContext;

    #[test]
    fn test_comparison_type() {
        assert_eq!(ComparisonType::new("Equal"), ComparisonType::Equal);
        assert_eq!(ComparisonType::new("NotEqual"), ComparisonType::NotEqual);
        assert_eq!(
            ComparisonType::new("GreaterThan"),
            ComparisonType::GreaterThan
        );
        assert_eq!(
            ComparisonType::new("GreaterOrEqual"),
            ComparisonType::GreaterOrEqual
        );
        assert_eq!(ComparisonType::new("LessThan"), ComparisonType::LessThan);
        assert_eq!(
            ComparisonType::new("LessOrEqual"),
            ComparisonType::LessOrEqual
        );
    }

    #[test]
    fn test_comparison_block_scalar() {
        let c = StubContext::default();
        let mut block = ComparisonBlock::<(f64, f64)>::default();
        let output = block.process(&Parameters::new("Equal"), &c, (1., 1.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("Equal"), &c, (0., 1.));
        assert_eq!(output, 0.0);

        let output = block.process(&Parameters::new("NotEqual"), &c, (1., 0.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("NotEqual"), &c, (1., 1.));
        assert_eq!(output, 0.0);

        // GreaterThan
        let output = block.process(&Parameters::new("GreaterThan"), &c, (1., 0.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("GreaterThan"), &c, (1., 1.));
        assert_eq!(output, 0.0);

        let output = block.process(&Parameters::new("GreaterThan"), &c, (0., 1.));
        assert_eq!(output, 0.0);

        // GreaterOrEqual
        let output = block.process(&Parameters::new("GreaterOrEqual"), &c, (1., 0.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("GreaterOrEqual"), &c, (1., 1.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("GreaterOrEqual"), &c, (0., 1.));
        assert_eq!(output, 0.0);

        // LessThan
        let output = block.process(&Parameters::new("LessThan"), &c, (0., 1.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("LessThan"), &c, (1., 1.));
        assert_eq!(output, 0.0);

        let output = block.process(&Parameters::new("LessThan"), &c, (1., 0.));
        assert_eq!(output, 0.0);

        // LessOrEqual
        let output = block.process(&Parameters::new("LessOrEqual"), &c, (0., 1.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("LessOrEqual"), &c, (1., 1.));
        assert_eq!(output, 1.0);

        let output = block.process(&Parameters::new("LessOrEqual"), &c, (1., 0.));
        assert_eq!(output, 0.0);
    }

    #[test]
    fn test_comparison_block_matrix() {
        let c = StubContext::default();
        let mut block = ComparisonBlock::<(Matrix<1, 3, f64>, Matrix<1, 3, f64>)>::default();
        let output = block.process(
            &Parameters::new("Equal"),
            &c,
            (
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
                &Matrix {
                    data: [[1.], [1.], [1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [0.], [0.]]
            }
        );

        let output = block.process(
            &Parameters::new("NotEqual"),
            &c,
            (
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
                &Matrix {
                    data: [[1.], [1.], [1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [1.], [1.]]
            }
        );

        let output = block.process(
            &Parameters::new("GreaterThan"),
            &c,
            (
                &Matrix {
                    data: [[1.], [1.], [-2.]],
                },
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [1.], [0.]]
            }
        );

        let output = block.process(
            &Parameters::new("GreaterOrEqual"),
            &c,
            (
                &Matrix {
                    data: [[1.], [1.], [-2.]],
                },
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [1.], [0.]]
            }
        );

        let output = block.process(
            &Parameters::new("LessThan"),
            &c,
            (
                &Matrix {
                    data: [[1.], [1.], [-2.]],
                },
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [0.], [1.]]
            }
        );

        let output = block.process(
            &Parameters::new("LessOrEqual"),
            &c,
            (
                &Matrix {
                    data: [[1.], [1.], [-2.]],
                },
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [0.], [1.]]
            }
        );
    }

    #[test]
    fn test_comparison_block_scalar_matrix() {
        let c = StubContext::default();
        let mut block = ComparisonBlock::<(f64, Matrix<1, 3, f64>)>::default();
        let output = block.process(
            &Parameters::new("Equal"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [0.], [0.]]
            }
        );

        let output = block.process(
            &Parameters::new("NotEqual"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[1.], [0.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [1.], [1.]]
            }
        );

        let output = block.process(
            &Parameters::new("GreaterThan"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[2.], [1.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [0.], [1.]]
            }
        );

        let output = block.process(
            &Parameters::new("GreaterOrEqual"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[2.], [1.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[0.], [1.], [1.]]
            }
        );

        let output = block.process(
            &Parameters::new("LessThan"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[2.], [1.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [0.], [0.]]
            }
        );

        let output = block.process(
            &Parameters::new("LessOrEqual"),
            &c,
            (
                1.,
                &Matrix {
                    data: [[2.], [1.], [-1.]],
                },
            ),
        );
        assert_eq!(
            output,
            &Matrix {
                data: [[1.], [1.], [0.]]
            }
        );
    }
}
