use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn count_calories(puzzle_lines: &[String], n: usize) -> Result<u64, Box<dyn Error>> {
    let mut data = vec![];
    let mut total = 0;

    for line in puzzle_lines.iter() {
        if line.is_empty() {
            data.push(total);
            total = 0;
            continue;
        }
        total += line.parse::<u64>()?
    }
    data.push(total);

    data.sort_by(|a, b| b.cmp(a));
    Ok(data.iter().take(n).sum::<u64>())
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

    let calories = count_calories(&puzzle_lines, 1)?;
    writeln!(stdout, "Answer Part 1 = {calories}")?;
    let calories = count_calories(&puzzle_lines, 3)?;
    writeln!(stdout, "Answer Part 2 = {calories}")?;

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
        assert_eq!(count_calories(&puzzle_lines, 1)?, 24000);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(count_calories(&puzzle_lines, 1)?, 68467);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(count_calories(&puzzle_lines, 3)?, 45000);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(count_calories(&puzzle_lines, 3)?, 203420);
        Ok(())
    }
}
