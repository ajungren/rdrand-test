use crate::arch::{_rdrand16_step, _rdrand32_step, _rdrand64_step};
use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::size_of;

const BITS_PER_BYTE: usize = 8;
const NIBBLES_PER_BYTE: usize = 2;

// * `Copy` is required to avoid cloning.
// * `Debug` and `Display` are required for convenience so types bound by
//   `RdRand` can be displayed and converted to strings.
// * `Eq` and `Hash` are required by `HashSet`.
// * `LowerHex` is required to format values with `{:x}`.
// * `UpperHex` is required to format values with `{:X}`.
//
// Because these traits are already implemented by u16, u32, and u64 there is no
// overhead from adding them to the bounds.
pub trait RdRand: Copy + Debug + Display + Eq + Hash + LowerHex + UpperHex {
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
    ($type:ty, $fn:ident) => {
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

#[derive(Clone, Copy, Debug)]
pub struct RdRandIter<T: RdRand> {
    // `RdRandIter` is stateless but generic types defined by `struct`s must be
    // used. Adding a `PhantomData<T>` field as a marker is sufficient. Because
    // `PhantomData<T>` is a ZST and `RdRandIter<T>` contains no other fields it
    // is also a ZST.
    //
    // Because `RdRandIter<T>` doesn't actually own any instances of `T`,
    // `PhantomData<*const T>` is used. (References require lifetimes which also
    // don't apply here, hence `*const T` is used instead of `&'_ T`.) See:
    // https://doc.rust-lang.org/stable/std/marker/struct.PhantomData.html#ownership-and-the-drop-check
    _phantom: PhantomData<*const T>,
}

impl<T: RdRand> Default for RdRandIter<T> {
    fn default() -> Self {
        RdRandIter {
            _phantom: Default::default(),
        }
    }
}

impl<T: RdRand> Iterator for RdRandIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(T::rdrand())
    }
}
