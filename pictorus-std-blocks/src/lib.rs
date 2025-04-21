mod fft_block;
pub use fft_block::FftBlock as FFTBlock;

mod system_time_block;
pub use system_time_block::{Sim, SystemTimeBlock};

mod traits {
    use corelib_traits::Pass;
    use nalgebra::{ComplexField, RealField, SimdPartialOrd};
    use num_traits::{AsPrimitive, Signed};
    pub trait Float:
        corelib_traits::Scalar
        + for<'a> Pass<By<'a> = Self>
        + PartialEq
        + nalgebra::Scalar
        + SimdPartialOrd
        + num_traits::Float
        + Signed
        + ComplexField
        + AsPrimitive<f64>
    where
        Self: RealField<RealField = Self>,
    {
    }

    impl Float for f32 {}
    impl Float for f64 {}
}
