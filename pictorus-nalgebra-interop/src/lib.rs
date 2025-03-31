//! Traits for inter-operation with nalgebra

// NOTE this only supports nalgebra 0.33.x

#![no_std]

use corelib_traits::Matrix;
use nalgebra::{Const, Dim, MatrixView, MatrixViewMut};

pub trait MatrixExt {
    type NROWS: Dim;
    type NCOLS: Dim;
    type Elem;

    fn as_view(&self) -> MatrixView<Self::Elem, Self::NROWS, Self::NCOLS>;
    fn as_view_mut(&mut self) -> MatrixViewMut<Self::Elem, Self::NROWS, Self::NCOLS>;
    fn from_view(view: &MatrixView<Self::Elem, Self::NROWS, Self::NCOLS>) -> Self;
}

impl<T, const NROWS: usize, const NCOLS: usize> MatrixExt for Matrix<NROWS, NCOLS, T>
where
    T: corelib_traits::Scalar + nalgebra::Scalar,
{
    type NROWS = Const<NROWS>;
    type NCOLS = Const<NCOLS>;
    type Elem = T;

    fn as_view(&self) -> MatrixView<Self::Elem, Self::NROWS, Self::NCOLS> {
        MatrixView::<Self::Elem, Self::NROWS, Self::NCOLS>::from_slice(self.data.as_flattened())
    }

    fn as_view_mut(&mut self) -> MatrixViewMut<Self::Elem, Self::NROWS, Self::NCOLS> {
        MatrixViewMut::<Self::Elem, Self::NROWS, Self::NCOLS>::from_slice(
            self.data.as_flattened_mut(),
        )
    }

    fn from_view(view: &MatrixView<Self::Elem, Self::NROWS, Self::NCOLS>) -> Self {
        let mut m = corelib_traits::Matrix::<NROWS, NCOLS, T>::zeroed();
        m.as_view_mut().copy_from(view);
        m
    }
}
