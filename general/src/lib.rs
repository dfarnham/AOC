use clap::{
    crate_description, crate_name, crate_version, value_parser, Arg, ArgMatches, ColorChoice,
    Command,
};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// splits and trims the input String on a separator character
// returns a Vec of parse::<T>() over the splits
pub fn trim_split_on<T>(text: &str, sep: char) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::str::FromStr>::Err: 'static,
{
    let mut parsed_splits = vec![];
    for s in text.split(sep) {
        parsed_splits.push(s.trim().parse::<T>()?)
    }
    Ok(parsed_splits)
}

// Reads the lines of a file, trims and returns them as a Vec of the supplied type
pub fn read_trimmed_data_lines<T>(
    filename: Option<&PathBuf>,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static,
    <T as FromStr>::Err: std::error::Error,
{
    let mut values = vec![];
    match filename {
        Some(file) if file.as_os_str() != "-" => {
            for line in read_lines(file)? {
                values.push(line?.trim().parse::<T>()?);
            }
            Ok(values)
        }
        _ => {
            // STDIN
            for line in io::BufReader::new(io::stdin()).lines() {
                values.push(line?.trim().parse::<T>()?);
            }
            Ok(values)
        }
    }
}

// Reads the lines of a file and returns them as a Vec of the supplied type
pub fn read_data_lines<T>(filename: Option<&PathBuf>) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static,
    <T as FromStr>::Err: std::error::Error,
{
    let mut values = vec![];
    match filename {
        Some(file) if file.as_os_str() != "-" => {
            for line in read_lines(file)? {
                values.push(line?.parse::<T>()?);
            }
            Ok(values)
        }
        _ => {
            // STDIN
            for line in io::BufReader::new(io::stdin()).lines() {
                values.push(line?.parse::<T>()?);
            }
            Ok(values)
        }
    }
}

// This should be called in cli apps
#[cfg(unix)]
pub fn reset_sigpipe() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)?;
        }
    }

    Ok(())
}
#[cfg(not(unix))]
pub fn reset_sigpipe() -> Result<(), Box<dyn std::error::Error>> {
    // no-op
}

// Simple clap-4 arg parser
pub fn get_args() -> ArgMatches {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .color(ColorChoice::Auto)
        .max_term_width(100)
        .arg(
            Arg::new("FILE")
                .short('i')
                .help("File to read, use '-' for standard input")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("time")
            .short('t')
            .help("Show runtime")
            .action(clap::ArgAction::SetTrue)
        );
    app.get_matches_from(env::args().collect::<Vec<String>>())
}
