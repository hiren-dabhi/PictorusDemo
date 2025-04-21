use corelib_traits::{Matrix, PassBy, ProcessBlock, Scalar};
use num_traits::Zero;
use utils::BlockData as OldBlockData;

/// An array of indices used to extract individual values from the input matrix. Invalid string values
/// will cause a panic when parsed.
pub struct Parameters<const N: usize> {
    indices: [usize; N],
}

impl<const N: usize> Parameters<N> {
    pub fn new<S: AsRef<str>>(index_values: &[S]) -> Self {
        let mut indices = [0; N];
        for (i, index) in index_values.iter().enumerate() {
            indices[i] = index.as_ref().split(":").last().unwrap().parse().expect(
                "Failed to parse index in VectorIndexBlock Parameters, check indices for validity",
            );
        }
        Parameters { indices }
    }
}

/// A block that extracts a set of values from an input matrix based on the linear index and outputs them in the
/// order of the indices. The output of the block matches the order of the indices, for example if the indices
/// are [15, 0], the 0th output of the block will be the 15th element of the input matrix and the 1st output of
/// the block will be the 0th element of the input matrix.
///
/// Note: Indices are 0 based and linear. If the indices is output side the bounds of the input matrix, the
/// output will be 0.
pub struct VectorIndexBlock<const N: usize, T, I>
where
    T: Scalar,
{
    pub data: [OldBlockData; N],
    buffer: [T; N],
    _phantom: core::marker::PhantomData<I>,
}

impl<const N: usize, T, I> Default for VectorIndexBlock<N, T, I>
where
    T: Scalar,
{
    fn default() -> Self {
        let initial = core::array::from_fn(|_f| OldBlockData::from_scalar(T::default().into()));
        VectorIndexBlock {
            data: initial,
            buffer: [T::default(); N],
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<const N: usize, const ROWS: usize, const COLS: usize, T> ProcessBlock
    for VectorIndexBlock<N, T, Matrix<ROWS, COLS, T>>
where
    T: Scalar + Zero,
{
    type Inputs = Matrix<ROWS, COLS, T>;
    type Output = [T; N];
    type Parameters = Parameters<N>;

    fn process(
        &mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: PassBy<'_, Self::Inputs>,
    ) -> PassBy<Self::Output> {
        // Linear array Index (i) -> Matrix Linear Index (x)
        // i = 0, x = 15 would represent output_0 of the block is the 15th linear element of the input matrix
        // i = 1, x = 0 would represent output_1 of the block is the 0th linear element of the input matrix
        let flattened = inputs.data.as_flattened();
        for (i, x) in parameters.indices.iter().enumerate() {
            // Check if the matrix index is within the bounds of the matrix dimensions, out-of-bounds indexes will
            // be set to 0.
            if *x < flattened.len() {
                let value = flattened[*x];
                self.buffer[i] = value;
                self.data[i] = OldBlockData::from_scalar(value.into());
            } else {
                self.buffer[i] = T::zero();
                self.data[i] = OldBlockData::from_scalar(T::zero().into());
            }
        }

        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::std::string::ToString;
    use crate::{vector_index_block::Parameters, VectorIndexBlock};
    use corelib_traits::{Matrix, ProcessBlock};
    use corelib_traits_testing::StubContext;
    use std::string::String;
    use std::vec;
    use std::vec::Vec;
    use utils::BlockData;

    #[test]
    fn test_vector_index_block_scalar() {
        let c = StubContext::default();
        let mut index_block = VectorIndexBlock::<1, f64, Matrix<3, 1, f64>>::default();
        let input = Matrix {
            data: [[1.0, 2.0, 3.0]],
        };

        // Codegen passes in index values like this:
        let vec_string_indexes: Vec<String> = vec![String::from("Scalar:2")];
        let parameters = Parameters::<1>::new(&vec_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output, &[3.0]);

        // This also works:
        let vec_string_indexes = vec!["2".to_string()];
        let parameters = Parameters::<1>::new(&vec_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output, &[3.0]);

        // And this:
        let vec_string_indexes = vec!["2"];
        let parameters = Parameters::<1>::new(&vec_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output, &[3.0]);

        // And this:
        let array_string_indexes = ["2"];
        let parameters = Parameters::<1>::new(&array_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output, &[3.0]);

        // And this:
        let array_string_indexes = ["Scalar:2"];
        let parameters = Parameters::<1>::new(&array_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output, &[3.0]);
    }

    #[test]
    fn test_vector_index_block_matrix() {
        let c = StubContext::default();
        let mut index_block = VectorIndexBlock::<2, f64, Matrix<2, 2, f64>>::default();
        let input = Matrix {
            data: [[5.0, 7.0], [6.0, 8.0]],
        };

        // An end user using the block would probably pass in values like this:
        let array_string_indexes = ["1", "3"];

        let parameters = Parameters::<2>::new(&array_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output[0], 7.0);
        assert_eq!(output[1], 8.0);
        assert_eq!(index_block.data[0].scalar(), 7.0);
        assert_eq!(index_block.data[0], BlockData::from_scalar(7.0));
        assert_eq!(index_block.data[1].scalar(), 8.0);
        assert_eq!(index_block.data[1], BlockData::from_scalar(8.0));
    }

    #[test]
    fn test_vector_index_block_input_too_short() {
        let c = StubContext::default();
        let mut index_block = VectorIndexBlock::<2, f64, Matrix<2, 2, f64>>::default();
        let input = Matrix {
            data: [[5.0, 7.0], [6.0, 8.0]],
        };

        // An end user using the block could also pass in values like this:
        let vec_string_indexes = vec!["1", "15"];

        let parameters = Parameters::<2>::new(&vec_string_indexes);
        let output = index_block.process(&parameters, &c, &input);
        assert_eq!(output[0], 7.0);
        assert_eq!(output[1], 0.0);
        assert_eq!(index_block.data[0].scalar(), 7.0);
        assert_eq!(index_block.data[0], BlockData::from_scalar(7.0));
        assert_eq!(index_block.data[1].scalar(), 0.0);
        assert_eq!(index_block.data[1], BlockData::from_scalar(0.0));
    }

    #[test]
    #[should_panic]
    fn test_vector_index_block_input_invalid_indices_negative() {
        // This test ensure that negative values are not allowed as indices. Since we allow users to input
        // indices as strings, we need to ensure that the user is alerted when attempting to parse a negative
        // value to a usize index.
        let array_string_indexes = ["Scalar:-1".to_string(), "-15".to_string()];
        let _parameters = Parameters::<2>::new(&array_string_indexes);
    }

    #[test]
    #[should_panic]
    fn test_vector_index_block_input_invalid_indices_string() {
        // Since we allow users to input indices as strings, we need to ensure that the user is alerted when
        // attempting to parse a non-number value to a usize index.
        let array_string_indexes = ["abc".to_string()];
        let _parameters = Parameters::<1>::new(&array_string_indexes);
    }

    #[test]
    #[should_panic]
    fn test_vector_index_block_input_invalid_indices_float() {
        // Since we allow users to input indices as strings, we need to ensure that the
        // user is alerted when attempting to parse a non-integer values to a usize index.
        let array_string_indexes = ["0.1234".to_string()];
        let _parameters = Parameters::<1>::new(&array_string_indexes);
    }
}
