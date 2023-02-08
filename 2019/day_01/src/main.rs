use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let fuel = |n| n / 3 - 2;

    Ok(puzzle_lines
        .iter()
        .map(|s| fuel(s.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
        .iter()
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let fuel = |n| n / 3 - 2;

    let mut data = vec![];
    for line in puzzle_lines.iter() {
        let mut n = fuel(line.parse::<i64>().unwrap());
        while n > 0 {
            data.push(n);
            n = fuel(n);
        }
    }
    Ok(data.iter().sum())
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
        assert_eq!(part1(&puzzle_lines)?, 34241);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 3495189);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 51316);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 5239910);
        Ok(())
    }
}
