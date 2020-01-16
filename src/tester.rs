use crate::arch::{__cpuid, __get_cpuid_max};
use std::collections::HashSet;

use bitflags::bitflags;

use crate::{Error, RdRand};

const CPUID_RDRAND_BIT: usize = 30;

bitflags! {
    #[derive(Default)]
    pub struct TesterOptions: u8 {
        const QUIET = 0b0000_0001;
        const SMOKE_TEST = 0b0000_0010;
    }
}

pub struct Tester {
    iterations: usize,
    terminal_width: usize,
    options: TesterOptions,
}

impl Tester {
    pub fn supported() -> bool {
        let max_leaf = unsafe { __get_cpuid_max(0).0 };
        if max_leaf >= 1 {
            let result = unsafe { __cpuid(1) };
            (result.ecx >> CPUID_RDRAND_BIT) & 1 == 1
        } else {
            false
        }
    }

    pub fn new(iterations: usize, options: TesterOptions) -> Result<Self, Error> {
        if Tester::supported() {
            let min_iterations = if options.contains(TesterOptions::SMOKE_TEST) {
                2
            } else {
                1
            };

            if iterations >= min_iterations {
                Ok(Tester {
                    iterations,
                    terminal_width: term_size::dimensions().unwrap_or((0, 0)).0,
                    options,
                })
            } else {
                Err(Error::InsufficientIterations {
                    required: min_iterations,
                })
            }
        } else {
            Err(Error::UnsupportedProcessor)
        }
    }

    #[inline]
    pub fn is_quiet(&self) -> bool {
        self.options.contains(TesterOptions::QUIET)
    }

    #[inline]
    pub fn is_smoke_test(&self) -> bool {
        self.options.contains(TesterOptions::SMOKE_TEST)
    }

    #[inline]
    pub fn run<T: RdRand>(&self) -> bool {
        print!("{bits}-bit RDRAND: ", bits = T::size_bits());
        if self.is_smoke_test() {
            self.smoke_test::<T>()
        } else {
            self.generate::<T>()
        }
    }

    fn generate<T: RdRand>(&self) -> bool {
        let mut duplicates = 0;
        let mut set = HashSet::new();
        let mut output_width = 0;

        print!("\n ");
        for value in T::iter_rdrand().take(self.iterations) {
            if set.get(&value).is_none() {
                set.insert(value);
            } else {
                duplicates += 1;
            }

            if !self.is_quiet() {
                let entry = format!(
                    " {value:0nibbles$x}",
                    value = value,
                    nibbles = T::size_nibbles()
                );
                output_width += entry.len();
                if output_width >= self.terminal_width {
                    print!("\n ");
                    if self.terminal_width > 0 {
                        output_width = 1 + entry.len();
                    }
                }
                print!("{}", entry);
            }
        }
        if !self.is_quiet() {
            print!("\n\n");
        }
        if duplicates > 0 {
            println!(
                "  {} / {} ({:.4}%) duplicate values",
                duplicates,
                self.iterations,
                f64::from(duplicates) / self.iterations as f64,
            );
        } else {
            println!("  No duplicate values");
        }

        true
    }

    fn smoke_test<T: RdRand>(&self) -> bool {
        let mut first_value = None;
        for value in T::iter_rdrand().take(self.iterations) {
            if let Some(first_value) = first_value {
                if first_value != value {
                    print!("OK");
                    return true;
                }
            } else {
                first_value = Some(value);
            }
        }

        let first_value = first_value.unwrap();
        print!(
            "0x{value:0nibbles$x} consecutively generated {iterations} times",
            value = first_value,
            nibbles = T::size_nibbles(),
            iterations = self.iterations,
        );
        false
    }
}
