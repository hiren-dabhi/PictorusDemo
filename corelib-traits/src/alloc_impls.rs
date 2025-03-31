use alloc::boxed::Box;

use crate::{sealed::Sealed, Pass, Scalar};

/// Runtime-sized matrix
// sadly this cannot be a DTO because the shape invariant needs to be preserved but we'll restrict
// the API to just a constructor; a teardown method and getters
#[derive(Clone, Debug, PartialEq)]
pub struct DMatrix<T>
where
    T: Scalar,
{
    data: Box<[T]>,
    shape: (usize, usize),
}

impl<T> DMatrix<T>
where
    T: Scalar,
{
    pub fn new(data: Box<[T]>, (nrows, ncols): (usize, usize)) -> Self {
        assert_eq!(data.len(), nrows * ncols);

        Self {
            data,
            shape: (nrows, ncols),
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn nrows(&self) -> usize {
        self.shape.0
    }

    pub fn ncols(&self) -> usize {
        self.shape.1
    }

    pub fn into_parts(self) -> (Box<[T]>, (usize, usize)) {
        (self.data, self.shape)
    }
}

impl<T> Pass for DMatrix<T>
where
    T: Scalar,
{
    type By<'a> = &'a Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl<T> Sealed for DMatrix<T> where T: Scalar {}
