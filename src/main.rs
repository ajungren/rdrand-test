extern crate rdrand_test;

use std::process::exit;
use std::str::FromStr;
use std::string::ToString;

use clap::{App, Arg};

use rdrand_test::{Error, Tester, TesterOptions};

const DEFAULT_ITERATIONS: &str = "500";
const SMOKE_TEST_ITERATIONS: &str = "4";

fn validate<T: FromStr>(value: String) -> Result<(), String>
where
    T::Err: ToString,
{
    match T::from_str(&value) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn run() -> Result<(), Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        // Flags
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Disables output of all generated values\n(Only outputs how many duplicate values were generated)")
        )
        .arg(Arg::with_name("smoke_test")
            .short("s")
            .long("smoke-test")
            .help("Runs a much shorter test, implies --quiet\n(Only verifies that the same number is not generated by consecutive RDRAND executions)")
        )
        // Options
        .arg(Arg::with_name("bits")
            .short("b")
            .long("bits")
            .takes_value(true)
            .possible_values(&["16", "32", "64"])
            .help("Runs test only for integers of the given size in bits")
        )
        // Arguments
        .arg(Arg::with_name("ITERATIONS")
            .default_value(DEFAULT_ITERATIONS)
            .default_value_if("smoke_test", None, SMOKE_TEST_ITERATIONS)
            .validator(validate::<usize>)
            .help("Number of iterations to run for each integer size")
        )
        .get_matches();

    let iterations = matches.value_of("ITERATIONS").unwrap();
    // `iterations` has already been validated by clap
    let iterations = usize::from_str(iterations).unwrap();
    let options = if matches.is_present("smoke_test") {
        TesterOptions::SMOKE_TEST
    } else if matches.is_present("quiet") {
        TesterOptions::QUIET
    } else {
        TesterOptions::default()
    };
    let tester = Tester::new(iterations, options)?;

    print!("Running {} iterations\n\n", iterations);
    let succeeded = if let Some(bits) = matches.value_of("bits") {
        // `bits` has already been validated by clap
        match bits {
            "16" => tester.run::<u16>(),
            "32" => tester.run::<u32>(),
            "64" => tester.run::<u64>(),
            _ => unreachable!(),
        }
    } else {
        let mut succeeded = true;
        succeeded &= tester.run::<u16>();
        println!();
        succeeded &= tester.run::<u32>();
        println!();
        succeeded &= tester.run::<u64>();
        succeeded
    };

    // `Tester::generate` does not output a trailing EOL after running.
    if tester.is_smoke_test() {
        println!();
    }

    // `255` is returned to distinguish RDRAND issues from `clap` errors.
    // (`1` is returned by the latter.)
    if !succeeded {
        exit(255);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(1);
    }
}
