use core::str::FromStr;
use corelib_traits::{Matrix, Pass, PassBy, ProcessBlock, Scalar};
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
            .expect("Codgen Error, this should never fail")
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

/// Performs an element-wise comparison operation on two inputs. Both inputs must be
/// of the same size and the output will be the same size as the inputs.
pub struct ComparisonBlock<T: Pass> {
    pub data: OldBlockData,
    buffer: Option<T>,
}

impl<T> Default for ComparisonBlock<T>
where
    T: Pass + Default,
    OldBlockData: FromPass<T>,
{
    fn default() -> Self {
        Self {
            data: <OldBlockData as FromPass<T>>::from_pass(T::default().as_by()),
            buffer: None,
        }
    }
}

macro_rules! impl_comparison_block {
    ($type:ty) => {
        impl ProcessBlock for ComparisonBlock<$type>
        where
            $type: Scalar,
        {
            type Inputs = ($type, $type);
            type Output = $type;
            type Parameters = Parameters;

            fn process(
                &mut self,
                parameters: &Self::Parameters,
                _context: &dyn corelib_traits::Context,
                input: PassBy<Self::Inputs>,
            ) -> PassBy<Self::Output> {
                let val = match parameters.comparison_type {
                    ComparisonType::Equal => input.0 == input.1,
                    ComparisonType::NotEqual => input.0 != input.1,
                    ComparisonType::GreaterThan => input.0 > input.1,
                    ComparisonType::GreaterOrEqual => input.0 >= input.1,
                    ComparisonType::LessThan => input.0 < input.1,
                    ComparisonType::LessOrEqual => input.0 <= input.1,
                };
                let output = self.buffer.insert(val.into());
                self.data = OldBlockData::from_scalar((*output).into());
                *output
            }
        }

        impl<const ROWS: usize, const COLS: usize> ProcessBlock
            for ComparisonBlock<Matrix<ROWS, COLS, $type>>
        where
            $type: Scalar,
            OldBlockData: FromPass<Matrix<ROWS, COLS, $type>>,
        {
            type Inputs = (Matrix<ROWS, COLS, $type>, Matrix<ROWS, COLS, $type>);
            type Output = Matrix<ROWS, COLS, $type>;
            type Parameters = Parameters;

            fn process(
                &mut self,
                parameters: &Self::Parameters,
                _context: &dyn corelib_traits::Context,
                input: PassBy<Self::Inputs>,
            ) -> PassBy<Self::Output> {
                let mut buffer = Matrix::<ROWS, COLS, $type>::zeroed();

                for r in 0..ROWS {
                    for c in 0..COLS {
                        let val = match parameters.comparison_type {
                            ComparisonType::Equal => input.0.data[c][r] == input.1.data[c][r],
                            ComparisonType::NotEqual => input.0.data[c][r] != input.1.data[c][r],
                            ComparisonType::GreaterThan => input.0.data[c][r] > input.1.data[c][r],
                            ComparisonType::GreaterOrEqual => {
                                input.0.data[c][r] >= input.1.data[c][r]
                            }
                            ComparisonType::LessThan => input.0.data[c][r] < input.1.data[c][r],
                            ComparisonType::LessOrEqual => input.0.data[c][r] <= input.1.data[c][r],
                        };
                        buffer.data[c][r] = val.into();
                    }
                }
                let output = self.buffer.insert(buffer);
                self.data = OldBlockData::from_pass(output);
                output
            }
        }
    };
}

impl_comparison_block!(i8);
impl_comparison_block!(u8);
impl_comparison_block!(i16);
impl_comparison_block!(u16);
impl_comparison_block!(i32);
impl_comparison_block!(u32);
impl_comparison_block!(f32);
impl_comparison_block!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    use corelib_traits_testing::StubContext;
    use num_traits::{One, Zero};
    use paste::paste;

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

    macro_rules! test_comparison_block {
        ($name:ident, $type:ty) => {
            paste! {
                #[test]
                fn [<test_comparison_block_scalars_ $name>]() {
                    /*
                    Runs through all the comparison types for scalars using inputs of
                    (2, 1) for the not equal case
                    (1, 1) for the equal case
                     */
                    let mut block = ComparisonBlock::<$type>::default();
                    let context = StubContext::default();
                    let input_ne = (<$type>::one() + <$type>::one(), <$type>::one());
                    let input_eq = (<$type>::one(), <$type>::one());

                    let mut parameters = Parameters::new("Equal");
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());

                    parameters.comparison_type = ComparisonType::NotEqual;
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());

                    parameters.comparison_type = ComparisonType::GreaterThan;
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());

                    parameters.comparison_type = ComparisonType::GreaterOrEqual;
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());

                    parameters.comparison_type = ComparisonType::LessThan;
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());

                    parameters.comparison_type = ComparisonType::LessOrEqual;
                    let output = block.process(&parameters, &context, input_ne);
                    assert_eq!(output, <$type>::zero().into());
                    assert_eq!(block.data.scalar(), <$type>::zero().into());
                    let output = block.process(&parameters, &context, input_eq);
                    assert_eq!(output, <$type>::one().into());
                    assert_eq!(block.data.scalar(), <$type>::one().into());
                }

                #[test]
                fn [<test_comparison_block_matrices_ $name>]() {
                    /*
                        This test runs through all the comparison types for matrices using two matricies:
                            [[1, 0], [0, 1]] and [[1, 1], [1, 1]]

                        They are combined into a tuple
                        i = (&m1, &m2)
                    */

                    let mut block = ComparisonBlock::<Matrix<2, 2, $type>>::default();
                    let context = StubContext::default();
                    let m1 = Matrix {
                        data: [[<$type>::one(), <$type>::zero()], [<$type>::zero(), <$type>::one()]],
                    };
                    let m2 = Matrix {
                        data: [[<$type>::one(), <$type>::one().into()], [<$type>::one(), <$type>::one()]],
                    };

                    let i = (&m1, &m2);

                    let mut parameters = Parameters::new("Equal");
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::one().into(), <$type>::zero().into()], [<$type>::zero().into(), <$type>::one().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::one().into(), <$type>::zero().into()], &[<$type>::zero().into(), <$type>::one().into()]]));

                    parameters.comparison_type = ComparisonType::NotEqual;
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::zero().into(), <$type>::one().into()], [<$type>::one().into(), <$type>::zero().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::zero().into(), <$type>::one().into()], &[<$type>::one().into(), <$type>::zero().into()]]));

                    parameters.comparison_type = ComparisonType::GreaterThan;
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::zero().into(), <$type>::zero().into()], [<$type>::zero().into(), <$type>::zero().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::zero().into(), <$type>::zero().into()], &[<$type>::zero().into(), <$type>::zero().into()]]));

                    parameters.comparison_type = ComparisonType::GreaterOrEqual;
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::one().into(), <$type>::zero().into()], [<$type>::zero().into(), <$type>::one().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::one().into(), <$type>::zero().into()], &[<$type>::zero().into(), <$type>::one().into()]]));

                    parameters.comparison_type = ComparisonType::LessThan;
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::zero().into(), <$type>::one().into()], [<$type>::one().into(), <$type>::zero().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::zero().into(), <$type>::one().into()], &[<$type>::one().into(), <$type>::zero().into()]]));

                    parameters.comparison_type = ComparisonType::LessOrEqual;
                    let output = block.process(&parameters, &context, i);
                    assert_eq!(*output, Matrix { data: [[<$type>::one().into(), <$type>::one().into()], [<$type>::one().into(), <$type>::one().into()]] });
                    assert_eq!(block.data, OldBlockData::from_matrix(&[&[<$type>::one().into(), <$type>::one().into()], &[<$type>::one().into(), <$type>::one().into()]]));

                }
            }
        };
    }

    test_comparison_block!(i8, i8);
    test_comparison_block!(u8, u8);
    test_comparison_block!(i16, i16);
    test_comparison_block!(u16, u16);
    test_comparison_block!(i32, i32);
    test_comparison_block!(u32, u32);
    test_comparison_block!(f32, f32);
    test_comparison_block!(f64, f64);
}
