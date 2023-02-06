use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;

// processes a list of Strings (trimmed line data) where
// an empty line represents a break between "groups" and
// each line in a group represents an individual response
//
// compute the number of unique chars in each group and
// return the sum of those char counts
fn solution1(data: &[String]) -> usize {
    let mut unique_responses = HashSet::<char>::new();
    let mut total = 0;

    for line in data {
        if line.is_empty() {
            total += unique_responses.len();
            unique_responses.clear();
            continue;
        }
        unique_responses.extend(HashSet::<char>::from_iter(line.chars()));
    }

    total + unique_responses.len()
}

// processes a list of Strings (trimmed line data) where
// an empty line represents a break between "groups" and
// each line in a group represents an individual response
//
// compute the shared chars accross all lines in
// each group and return the sum of those counts
fn solution2(data: &[String]) -> usize {
    let mut shared_responses = HashSet::<char>::new();
    let mut total = 0;
    let mut begin_new_group = true;

    for line in data {
        if line.is_empty() {
            total += shared_responses.len();
            shared_responses.clear();
            begin_new_group = true;
            continue;
        }

        let charset = HashSet::<char>::from_iter(line.chars());
        shared_responses = match begin_new_group {
            true => charset,
            false => HashSet::<char>::from_iter(shared_responses.intersection(&charset).copied()),
        };
        begin_new_group = false;
    }

    total + shared_responses.len()
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

    writeln!(stdout, "Answer Part 1 = {}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&puzzle_lines))?;

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
        assert_eq!(11, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(6585, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(6, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(3276, solution2(&data));
    }
}
