use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String], n: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    trim_split_on::<usize>(&puzzle_lines[n], '-')
}

// must be 6 ordered digits (every digit must be less than or equal to the following digit)
fn compare_digits(s: &str) -> bool {
    // use a running window of size 2 to compare the values of successive pairs
    // the count of x > y will be 0 if the digits are ordered
    s.len() == 6
        && s.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .filter(|n| n[0] > n[1])
            .count()
            == 0
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let range = get_data(puzzle_lines, 0)?;
    let (low, high) = (range[0], range[1]);
    let double_digit_re = Regex::new(r"00|11|22|33|44|55|66|77|88|99").unwrap();

    // return filtered count
    Ok((low..high)
        // string representation of the integer
        .map(|n| n.to_string())
        // the string has to contain a doubled digit
        .filter(|s| double_digit_re.is_match(s))
        // must be 6 ordered digits (every digit must be less than or equal to the following digit)
        .filter(|s| compare_digits(s))
        .count())
}

// same as part1() but using String.contains() instead of Regex.is_match()
fn part1_contains(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let range = get_data(puzzle_lines, 0)?;
    let (low, high) = (range[0], range[1]);

    let doubled_digit = |s: &str| {
        s.contains("00")
            || s.contains("11")
            || s.contains("22")
            || s.contains("33")
            || s.contains("44")
            || s.contains("55")
            || s.contains("66")
            || s.contains("77")
            || s.contains("88")
            || s.contains("99")
    };

    // return filtered count
    Ok((low..high)
        // string representation of the integer
        .map(|n| n.to_string())
        // the string has to contain a doubled digit
        .filter(|s| doubled_digit(s))
        // must be 6 ordered digits (every digit must be less than or equal to the following digit)
        .filter(|s| compare_digits(s))
        .count())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let range = get_data(puzzle_lines, 0)?;
    let (low, high) = (range[0], range[1]);

    let doubled_digit_only = |s: &str| {
        s.contains("00") && !s.contains("000")
            || s.contains("11") && !s.contains("111")
            || s.contains("22") && !s.contains("222")
            || s.contains("33") && !s.contains("333")
            || s.contains("44") && !s.contains("444")
            || s.contains("55") && !s.contains("555")
            || s.contains("66") && !s.contains("666")
            || s.contains("77") && !s.contains("777")
            || s.contains("88") && !s.contains("888")
            || s.contains("99") && !s.contains("999")
    };

    // return filtered count
    Ok((low..high)
        // string representation of the integer
        .map(|n| n.to_string())
        // the string must contain a doubled digit which isn't a subset of a larger sequence
        .filter(|s| doubled_digit_only(s))
        // must be 6 ordered digits (every digit must be less than or equal to the following digit)
        .filter(|s| compare_digits(s))
        .count())
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines)?)?;
    assert_eq!(part1(&puzzle_lines)?, part1_contains(&puzzle_lines)?);

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
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 1660);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1660);
        Ok(())
    }

    #[test]
    fn part1_c() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1_contains(&puzzle_lines)?, 1660);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 1135);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1135);
        Ok(())
    }
}
