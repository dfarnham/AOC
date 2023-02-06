use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (u64, u64) {
    (data[0].parse::<u64>().unwrap(), data[1].parse::<u64>().unwrap())
}

fn transform_n(n: u64, loops: usize) -> u64 {
    let mut result = 1;
    for _ in 0..loops {
        result = (result * n) % 20201227;
    }
    result
}

fn solution1(data: &[String]) -> u64 {
    let (pubkey1, pubkey2) = get_data(data);

    let mut i = 1;
    let mut result = 1;
    loop {
        result = (result * 7) % 20201227;
        if result == pubkey1 {
            return transform_n(pubkey2, i);
        } else if result == pubkey2 {
            return transform_n(pubkey1, i);
        }
        i += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_trimmed_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(14897079, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(8329514, solution1(&data));
    }
}
