use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

const SPELLED_NUMBERS: &str = "one two three four five six seven eight nine";

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, false)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, true)
}

fn solve(puzzle_lines: &[String], p2: bool) -> Result<usize, Box<dyn Error>> {
    // a map of digits in string form to digit values
    let mut numbers = HashMap::<_, _>::from_iter((1..=9).map(|c| (c.to_string(), c)));

    // add spelled numbers to the map for part2
    if p2 {
        for (i, spelled_number) in SPELLED_NUMBERS.split_whitespace().enumerate() {
            numbers.insert(spelled_number.into(), i + 1);
        }
    }

    Ok(puzzle_lines
        .iter()
        .map(|line| {
            // test all the line slices for hash keys e.g. ["1", "2", ..., "eight", "nine"]
            let found_keys: Vec<_> = (0..line.len())
                .filter_map(|i| numbers.keys().find(|k| line[i..].starts_with(*k)))
                .collect();

            // take the (first, last) keys from `found_keys` to form a 2-digit number and emit
            // Note: list.first(), list.last() correctly references the same element on 1-element list
            match (found_keys.first(), found_keys.last()) {
                (Some(first), Some(last)) => numbers[*first] * 10 + numbers[*last],
                _ => 0,
            }
        })
        .sum())
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

    let n = part1(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = part2(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 2 = {n}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file = std::path::PathBuf::from(filename);
        Ok(read_trimmed_data_lines(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part1(&puzzle_lines)?, 142);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part1(&puzzle_lines)?, 209);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 55090);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part2(&puzzle_lines)?, 281);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 54845);
        Ok(())
    }
}
