use corelib_traits::{Matrix, Pass, PassBy, ProcessBlock, Promote, Promotion, Scalar};
use utils::{BlockData as OldBlockData, FromPass};

pub struct BiasBlock<B, T>
where
    B: Scalar,
    T: Apply<B>,
{
    pub data: OldBlockData,
    buffer: Option<T::Output>,
}

impl<B, T> Default for BiasBlock<B, T>
where
    B: Scalar,
    T: Apply<B>,
{
    fn default() -> Self {
        Self {
            data: OldBlockData::from_scalar(0.0),
            buffer: None,
        }
    }
}

impl<B, T> ProcessBlock for BiasBlock<B, T>
where
    B: Scalar,
    T: Apply<B>,
    OldBlockData: FromPass<T::Output>,
{
    type Inputs = T;
    type Output = T::Output;
    type Parameters = Parameters<B>;

    fn process(
        &mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        input: PassBy<Self::Inputs>,
    ) -> PassBy<Self::Output> {
        let output = T::apply(&mut self.buffer, input, parameters.bias);
        self.data = OldBlockData::from_pass(output);
        output
    }
}

pub trait Apply<B: Scalar>: Pass {
    type Output: Pass;

    fn apply<'s>(
        store: &'s mut Option<Self::Output>,
        input: PassBy<Self>,
        bias: B,
    ) -> PassBy<'s, Self::Output>;
}

impl<B> Apply<B> for f64
where
    B: Promote<f64> + Scalar,
{
    type Output = Promotion<B, f64>;

    fn apply<'s>(
        store: &'s mut Option<Self::Output>,
        input: PassBy<Self>,
        bias: B,
    ) -> PassBy<'s, Self::Output> {
        let output =
            <B as Promote<f64>>::promote_left(bias) + <B as Promote<f64>>::promote_right(input);
        *store = Some(output);
        output
    }
}

impl<B> Apply<B> for f32
where
    B: Promote<f32> + Scalar,
{
    type Output = Promotion<B, f32>;

    fn apply<'s>(
        store: &'s mut Option<Self::Output>,
        input: PassBy<Self>,
        bias: B,
    ) -> PassBy<'s, Self::Output> {
        let output =
            <B as Promote<f32>>::promote_left(bias) + <B as Promote<f32>>::promote_right(input);
        *store = Some(output);
        output
    }
}

impl<const NROWS: usize, const NCOLS: usize, B, T> Apply<B> for Matrix<NROWS, NCOLS, T>
where
    T: Scalar,
    B: Promote<T>,
{
    type Output = Matrix<NROWS, NCOLS, Promotion<B, T>>;

    fn apply<'s>(
        store: &'s mut Option<Self::Output>,
        input: PassBy<Self>,
        bias: B,
    ) -> PassBy<'s, Self::Output> {
        let output = store.insert(Matrix::zeroed());
        for i in 0..NROWS {
            for j in 0..NCOLS {
                output.data[j][i] = <B as Promote<T>>::promote_left(bias)
                    + <B as Promote<T>>::promote_right(input.data[j][i]);
            }
        }
        output
    }
}

pub struct Parameters<B>
where
    B: Scalar,
{
    pub bias: B,
}

impl<B> Parameters<B>
where
    B: Scalar,
{
    pub fn new(bias: B) -> Self {
        Self { bias }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use corelib_traits_testing::StubContext;
    use utils::ToPass;

    #[test]
    fn test_bias_scalar() {
        let mut block = BiasBlock::<f64, f64>::default();
        let parameters = Parameters::new(3.0);
        let context = StubContext::default();

        let output = block.process(&parameters, &context, 2.0);
        assert_eq!(output, 5.0);
        assert_eq!(block.data.scalar(), 5.0);
    }

    #[test]
    fn test_bias_scalar_to_pass() {
        let mut block = BiasBlock::<f64, f64>::default();
        let parameters = Parameters::new(3.0);
        let context = StubContext::default();
        let input = OldBlockData::from_scalar(-3.1);

        let output = block.process(&parameters, &context, input.to_pass());
        assert_relative_eq!(output, -0.1);
        assert_relative_eq!(block.data.scalar(), -0.1);
    }

    #[test]
    fn test_bias_matrix() {
        let mut block = BiasBlock::<f64, Matrix<2, 2, f64>>::default();
        let context = StubContext::default();
        let input = Matrix {
            data: [[1.0, 2.0], [3.0, 4.0]],
        };
        let parameters = Parameters::new(2.0);
        let output = block.process(&parameters, &context, &input);
        assert_eq!(output.data, [[3.0, 4.0], [5.0, 6.0]]);
        assert_eq!(
            block.data.get_data().as_slice(),
            [[3.0, 4.0], [5.0, 6.0]].as_flattened()
        );
    }

    #[test]
    fn test_bias_matrix_to_pass() {
        let mut block = BiasBlock::<f64, Matrix<2, 2, f64>>::default();
        let context = StubContext::default();
        let input = OldBlockData::from_matrix(&[&[1.0, 3.0], &[2.0, 4.0]]);
        let parameters = Parameters::new(2.0);
        let output = block.process(&parameters, &context, &input.to_pass());
        assert_eq!(output.data, [[3.0, 4.0], [5.0, 6.0]]);
        assert_eq!(
            block.data.get_data().as_slice(),
            [[3.0, 4.0], [5.0, 6.0]].as_flattened()
        );
    }
}
