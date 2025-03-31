#![no_std]
// and conditionally no_alloc

#[cfg(feature = "alloc")]
extern crate alloc;

use core::cell::Ref;
use core::mem;
use core::time::Duration;

#[cfg(feature = "alloc")]
pub use alloc_impls::DMatrix;
use hal::{GpioOutputBlock, SerialReceiveBlock, SerialTransmitBlock};
use sealed::Sealed;
#[cfg(feature = "alloc")]
pub use transition_impls::BlockData;

#[cfg(feature = "alloc")]
mod alloc_impls;
pub mod hal;
mod sealed;
#[cfg(feature = "alloc")]
mod transition_impls;

/// A state is a collection of blocks with transitions and without inputs and outputs
pub trait State<'m>: BlocksNew<'m> {
    type NextState;

    fn run(&mut self, context: &dyn Context) -> Option<Self::NextState>;
}

/// A component is a collection of blocks with optional inputs and outputs
pub trait Component<'m>: BlocksNew<'m> {
    type Inputs: Pass + ?Sized;
    type Output: Pass + ?Sized;

    fn run<'c>(
        &'c mut self,
        context: &dyn Context,
        inputs: PassBy<'_, Self::Inputs>,
    ) -> PassBy<'c, Self::Output>;

    /// hook to perform any resource clean-up
    fn on_tick_end(&mut self) {}
}

/// Constructor method for a *group* of blocks
pub trait BlocksNew<'m> {
    fn new(device_manager: &'m impl DeviceManager) -> Self;
}

/// A processing block
pub trait ProcessBlock: Default {
    // NOTE because of the `Inputs` trait bound; all blocks must have at least *one* input
    type Inputs: Pass + ?Sized;
    type Output: Pass + ?Sized;
    type Parameters;

    fn process<'b>(
        &'b mut self,
        parameters: &Self::Parameters,
        context: &dyn Context,
        inputs: PassBy<'_, Self::Inputs>,
    ) -> PassBy<'b, Self::Output>;
}

/// Updates a block parameter
pub trait Update {
    type Parameter;

    fn update(&mut self, parameter: Self::Parameter);
}

/// A generator block
///
/// This block has no inputs
pub trait GeneratorBlock: Default {
    type Parameters;
    type Output: Pass + ?Sized;

    fn generate(
        &mut self,
        parameters: &Self::Parameters,
        context: &dyn Context,
    ) -> PassBy<Self::Output>;
}

/// An output block
///
/// This block has no output and usually performs a "side effect" instead of outputting data
pub trait OutputBlock {
    type Inputs: Pass + ?Sized;

    fn output(&self, context: &dyn Context, inputs: PassBy<'_, Self::Inputs>);
}

/// An input block
///
/// This block has no inputs. Unlike a `GeneratorBlock` it outputs data from the real world rather
/// than synthetic data.
pub trait InputBlock {
    type Output: Lend + ?Sized;

    fn input(&self, context: &dyn Context) -> LendAs<'_, Self::Output>;
}

/// Application-specific "device" manager
pub trait DeviceManager {
    /// Initializes hardware according to device tree information
    fn new() -> Self;

    fn gpio_output_block(&self, _index: usize) -> Option<GpioOutputBlock<'_>> {
        None
    }

    fn serial_receive_block(&self, _index: usize) -> Option<SerialReceiveBlock<'_>> {
        None
    }

    fn serial_transmit_block(&self, _index: usize) -> Option<SerialTransmitBlock<'_>> {
        None
    }

    /// Runs at the start of each tick
    fn on_tick_start(&self);
    /// Runs at the end of each tick
    fn on_tick_end(&self);
}

/// The execution context
// this trait avoids leaking types associated to the "runtime" into the signature of
// `{Block,Generator}::run`
pub trait Context {
    // XXX should the return type be an `Option`? the very first call of `timestep` returns
    // `None` and all the subsequent calls return `Some`
    fn timestep(&self) -> Duration;
    /// Time elapsed since the start of the program / simulation
    fn time(&self) -> Duration;
}

pub trait DurationExt<T> {
    fn as_sec_float(&self) -> T;
}

impl DurationExt<f32> for Duration {
    fn as_sec_float(&self) -> f32 {
        self.as_secs_f32()
    }
}

impl DurationExt<f64> for Duration {
    fn as_sec_float(&self) -> f64 {
        self.as_secs_f64()
    }
}

/// Data can be passed between blocks
pub trait Pass: Sealed + 'static {
    /// Whether the data is passed by value or by reference
    type By<'a>: Copy;

    fn as_by(&self) -> Self::By<'_>;
}

pub type PassBy<'a, T> = <T as Pass>::By<'a>;

impl<T> Pass for T
where
    T: Scalar,
{
    type By<'a> = Self;

    fn as_by(&self) -> Self::By<'_> {
        *self
    }
}

/// "Scalar" types
///
/// Marker trait for small primitives like floats, integers and booleans
pub trait Scalar: Sealed + Copy + 'static + Default + Into<f64> {}

impl Scalar for bool {}
impl Sealed for bool {}

impl Scalar for u8 {}
impl Sealed for u8 {}

impl Scalar for i8 {}
impl Sealed for i8 {}

impl Scalar for u16 {}
impl Sealed for u16 {}

impl Scalar for i16 {}
impl Sealed for i16 {}

impl Scalar for u32 {}
impl Sealed for u32 {}

impl Scalar for i32 {}
impl Sealed for i32 {}

impl Scalar for f32 {}
impl Sealed for f32 {}

impl Scalar for f64 {}
impl Sealed for f64 {}

/// Auto-promotion
pub trait Promote<RHS: Scalar>: Scalar {
    type Output: Scalar
        + core::ops::Add<Output = Self::Output>
        + core::ops::Mul<Output = Self::Output>
        + core::ops::Sub<Output = Self::Output>
        + core::ops::Div<Output = Self::Output>;

    fn promote_left(self) -> Self::Output;
    fn promote_right(rhs: RHS) -> Self::Output;
}

macro_rules! promotions {
    ($( ( $($from:ident),* ) -> $to:ident ),*) => {
        $(
            impl Promote<$to> for $to {
                type Output = $to;

                fn promote_left(self) -> Self::Output {
                    self
                }

                fn promote_right(rhs: $to) -> Self::Output {
                    rhs
                }
            }

            $(
                impl Promote<$from> for $to {
                    type Output = $to;

                    fn promote_left(self) -> Self::Output {
                        self
                    }

                    fn promote_right(rhs: $from) -> Self::Output {
                        rhs as $to
                    }
                }

                impl Promote<$to> for $from {
                    type Output = $to;

                    fn promote_left(self) -> Self::Output {
                        self as $to
                    }

                    fn promote_right(rhs: $to) -> Self::Output {
                        rhs
                    }
                }
            )*
        )*
    };
}

// TODO add more impls are needed
promotions! {
    (u8, u16) -> f32,
    (f32) -> f64
}

pub type Promotion<L, R> = <L as Promote<R>>::Output;

// a fixed-size array is like a mathematical vector
// NOTE the `Pod` trait bound prevents the creation of nested vectors
impl<const N: usize, T> Pass for [T; N]
where
    T: Scalar,
{
    type By<'o> = &'o Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl<const N: usize, T> Sealed for [T; N] where T: Scalar {}

impl Pass for [u8] {
    type By<'a> = &'a Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl Sealed for [u8] {}

/// Like `Pass` but allows returning `cell::Ref` from `InputBlock`s
pub trait Lend: Sealed + 'static {
    /// How the data is loaned
    type As<'a>;
}

pub type LendAs<'a, T> = <T as Lend>::As<'a>;

impl Lend for [u8] {
    type As<'a> = Ref<'a, Self>;
}

/// Matrix in column-major order
// this type is only used as a "DTO" (Data Transfer Object). meaning that this type has
// no methods and it does NOT enforce any sort of invariants (which is why its field is public).
// it's only used to *transfer* data between blocks
//
// utility functions to convert between this and third-party crates like `nalgebra` are to be
// provided in a different crate. this also keeps this "interface" crate free from
// third-party dependencies
// XXX should this encode row-order vs column-order?
// NOTE the `Scalar` trait bound prevents the creation of nested matrices
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const NROWS: usize, const NCOLS: usize, T>
where
    T: Scalar,
{
    pub data: [[T; NROWS]; NCOLS],
}

impl<const NROWS: usize, const NCOLS: usize, T> Default for Matrix<NROWS, NCOLS, T>
where
    T: Scalar,
{
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<const NROWS: usize, const NCOLS: usize, T> Matrix<NROWS, NCOLS, T>
where
    T: Scalar,
{
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Matrix<NROWS, NCOLS, U>
    where
        U: Scalar,
    {
        Matrix {
            data: self.data.map(|col| col.map(&mut f)),
        }
    }

    pub fn zeroed() -> Self {
        // SAFETY: `T: Scalar` is "sealed" so we know all the types it could be instantiated to and
        // we also know they are all primitives / "plain old data" so all bits set to zero is
        // a valid representation
        Self {
            data: unsafe { mem::zeroed() },
        }
    }
}

impl<const NROWS: usize, const NCOLS: usize, T> Pass for Matrix<NROWS, NCOLS, T>
where
    T: Scalar,
{
    type By<'a> = &'a Self;

    fn as_by(&self) -> Self::By<'_> {
        self
    }
}

impl<const NROWS: usize, const NCOLS: usize, T> Sealed for Matrix<NROWS, NCOLS, T> where T: Scalar {}

impl Pass for () {
    type By<'a> = Self;

    fn as_by(&self) -> Self::By<'_> {
        *self
    }
}

impl Sealed for () {}

impl<A, B> Pass for (A, B)
where
    A: Pass,
    B: Pass + ?Sized,
{
    type By<'a> = (PassBy<'a, A>, PassBy<'a, B>);

    fn as_by(&self) -> Self::By<'_> {
        (self.0.as_by(), self.1.as_by())
    }
}

impl<A, B> Sealed for (A, B)
where
    A: Pass,
    B: Pass + ?Sized,
{
}

impl<A, B, C> Pass for (A, B, C)
where
    A: Pass,
    B: Pass,
    C: Pass + ?Sized,
{
    type By<'a> = (PassBy<'a, A>, PassBy<'a, B>, PassBy<'a, C>);

    fn as_by(&self) -> Self::By<'_> {
        (self.0.as_by(), self.1.as_by(), self.2.as_by())
    }
}
impl<A, B, C> Sealed for (A, B, C)
where
    A: Pass,
    B: Pass,
    C: Pass + ?Sized,
{
}

impl<A, B, C, D> Pass for (A, B, C, D)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass + ?Sized,
{
    type By<'a> = (PassBy<'a, A>, PassBy<'a, B>, PassBy<'a, C>, PassBy<'a, D>);

    fn as_by(&self) -> Self::By<'_> {
        (
            self.0.as_by(),
            self.1.as_by(),
            self.2.as_by(),
            self.3.as_by(),
        )
    }
}
impl<A, B, C, D> Sealed for (A, B, C, D)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass + ?Sized,
{
}

impl<A, B, C, D, E> Pass for (A, B, C, D, E)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass + ?Sized,
{
    type By<'a> = (
        PassBy<'a, A>,
        PassBy<'a, B>,
        PassBy<'a, C>,
        PassBy<'a, D>,
        PassBy<'a, E>,
    );

    fn as_by(&self) -> Self::By<'_> {
        (
            self.0.as_by(),
            self.1.as_by(),
            self.2.as_by(),
            self.3.as_by(),
            self.4.as_by(),
        )
    }
}
impl<A, B, C, D, E> Sealed for (A, B, C, D, E)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass + ?Sized,
{
}

impl<A, B, C, D, E, F> Pass for (A, B, C, D, E, F)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass + ?Sized,
{
    type By<'a> = (
        PassBy<'a, A>,
        PassBy<'a, B>,
        PassBy<'a, C>,
        PassBy<'a, D>,
        PassBy<'a, E>,
        PassBy<'a, F>,
    );

    fn as_by(&self) -> Self::By<'_> {
        (
            self.0.as_by(),
            self.1.as_by(),
            self.2.as_by(),
            self.3.as_by(),
            self.4.as_by(),
            self.5.as_by(),
        )
    }
}
impl<A, B, C, D, E, F> Sealed for (A, B, C, D, E, F)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass + ?Sized,
{
}

impl<A, B, C, D, E, F, G> Pass for (A, B, C, D, E, F, G)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass,
    G: Pass + ?Sized,
{
    type By<'a> = (
        PassBy<'a, A>,
        PassBy<'a, B>,
        PassBy<'a, C>,
        PassBy<'a, D>,
        PassBy<'a, E>,
        PassBy<'a, F>,
        PassBy<'a, G>,
    );

    fn as_by(&self) -> Self::By<'_> {
        (
            self.0.as_by(),
            self.1.as_by(),
            self.2.as_by(),
            self.3.as_by(),
            self.4.as_by(),
            self.5.as_by(),
            self.6.as_by(),
        )
    }
}
impl<A, B, C, D, E, F, G> Sealed for (A, B, C, D, E, F, G)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass,
    G: Pass + ?Sized,
{
}

impl<A, B, C, D, E, F, G, H> Pass for (A, B, C, D, E, F, G, H)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass,
    G: Pass,
    H: Pass + ?Sized,
{
    type By<'a> = (
        PassBy<'a, A>,
        PassBy<'a, B>,
        PassBy<'a, C>,
        PassBy<'a, D>,
        PassBy<'a, E>,
        PassBy<'a, F>,
        PassBy<'a, G>,
        PassBy<'a, H>,
    );

    fn as_by(&self) -> Self::By<'_> {
        (
            self.0.as_by(),
            self.1.as_by(),
            self.2.as_by(),
            self.3.as_by(),
            self.4.as_by(),
            self.5.as_by(),
            self.6.as_by(),
            self.7.as_by(),
        )
    }
}
impl<A, B, C, D, E, F, G, H> Sealed for (A, B, C, D, E, F, G, H)
where
    A: Pass,
    B: Pass,
    C: Pass,
    D: Pass,
    E: Pass,
    F: Pass,
    G: Pass,
    H: Pass + ?Sized,
{
}
