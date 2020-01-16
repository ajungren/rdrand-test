#[cfg(target_arch = "x86")]
pub(crate) use std::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
pub(crate) use std::arch::x86_64 as arch;

mod error;
mod rdrand;
mod tester;

pub use crate::error::Error;
pub use crate::rdrand::{RdRand, RdRandIter};
pub use crate::tester::{Tester, TesterOptions};
