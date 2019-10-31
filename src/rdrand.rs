#[cfg(target_arch = "x86")]
use std::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64 as arch;

use arch::{_rdrand16_step, _rdrand32_step, _rdrand64_step};
use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::size_of;

const BITS_PER_BYTE: usize = 8;
const NIBBLES_PER_BYTE: usize = 2;

// * `Copy` is required to avoid cloning and is implemented by all primitive
//   integer types.
// * `Default` is required to create `RdRandIter` instances with
//   `Default::default`.
// *  `Eq` and `Hash` are required by `HashSet`.
// * `LowerHex` is required to format values with `{:x}`.
// * `UpperHex` is required to format values with `{:X}`.
pub trait RdRand: Copy + Default + Debug + Display + Eq + Hash + LowerHex + UpperHex {
    #[inline]
    fn size_bits() -> usize {
        size_of::<Self>() * BITS_PER_BYTE
    }

    #[inline]
    fn size_nibbles() -> usize {
        size_of::<Self>() * NIBBLES_PER_BYTE
    }

    #[inline]
    fn iter_rdrand() -> RdRandIter<Self> {
        Default::default()
    }

    fn rdrand() -> Self;
}

macro_rules! impl_rdrand {
    // $type must be an `ident` rather than `ty` for `std::$type::MAX` to work.
    ($type:ident, $fn:ident) => {
        impl RdRand for $type {
            fn rdrand() -> Self {
                let mut value = 0;
                assert_eq!(1, unsafe { $fn(&mut value) });
                value
            }
        }
    };
}

impl_rdrand!(u16, _rdrand16_step);
impl_rdrand!(u32, _rdrand32_step);
impl_rdrand!(u64, _rdrand64_step);

#[derive(Clone, Copy, Debug, Default)]
pub struct RdRandIter<T: RdRand> {
    // `RdRandIter` is stateless but generic types defined by `struct`s must be
    // used. Adding a `PhantomData` field as a marker is sufficient.
    _phantom: PhantomData<T>,
}

impl<T: RdRand> Iterator for RdRandIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(T::rdrand())
    }
}
