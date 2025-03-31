use core::cell::Ref;

use alloc::vec::Vec;

use crate::{sealed::Sealed, DMatrix, Pass};

#[derive(Clone, Debug, PartialEq)]
pub enum BlockData {
    Bool(bool),
    Bytes(Vec<u8>),
    Float(f64),
    Matrix(DMatrix<f64>),
    Vector(Vec<f64>),
}

impl From<Ref<'_, [u8]>> for BlockData {
    fn from(value: Ref<'_, [u8]>) -> Self {
        BlockData::Bytes(value.to_vec())
    }
}

impl From<Vec<f64>> for BlockData {
    fn from(v: Vec<f64>) -> Self {
        Self::Vector(v)
    }
}

impl From<DMatrix<f64>> for BlockData {
    fn from(v: DMatrix<f64>) -> Self {
        Self::Matrix(v)
    }
}

impl From<f64> for BlockData {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<Vec<u8>> for BlockData {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl From<bool> for BlockData {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl BlockData {
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        if let Self::Bytes(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Self::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_matrix(&self) -> Option<&DMatrix<f64>> {
        if let Self::Matrix(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_vector(&self) -> Option<&[f64]> {
        if let Self::Vector(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the block data is [`Float`].
    ///
    /// [`Float`]: BlockData::Float
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }
}

impl Pass for BlockData {
    type By<'a> = &'a Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl Pass for [BlockData] {
    type By<'a> = &'a Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl Sealed for BlockData {}

impl Sealed for [BlockData] {}
