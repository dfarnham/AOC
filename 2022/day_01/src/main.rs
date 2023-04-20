use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use itertools::Itertools;


// itertools gem from https://fasterthanli.me/series/advent-of-code-2022/part-1
fn count_calories<T>(puzzle_lines: &[String], n: usize) -> Result<T, Box<dyn Error>>
where
    T: std::str::FromStr + std::iter::Sum + std::cmp::Ord + std::fmt::Debug,
{
    Ok(puzzle_lines.iter()
        .map(|s| s.parse::<T>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<T>())
        .map(std::cmp::Reverse)
        .k_smallest(n)
        .map(|x| x.0)
        .sum::<T>())
}

#[allow(dead_code)]
fn count_calories_orig(puzzle_lines: &[String], n: usize) -> Result<u64, Box<dyn Error>> {
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

    let calories = count_calories::<u64>(&puzzle_lines, 1)?;
    writeln!(stdout, "Answer Part 1 = {calories}")?;
    let calories = count_calories::<u64>(&puzzle_lines, 3)?;
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
        assert_eq!(count_calories::<u64>(&puzzle_lines, 1)?, 24000);
        assert_eq!(count_calories::<u64>(&puzzle_lines, 1)?, count_calories_orig(&puzzle_lines, 1)?);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(count_calories::<u64>(&puzzle_lines, 1)?, 68467);
        assert_eq!(count_calories::<u64>(&puzzle_lines, 1)?, count_calories_orig(&puzzle_lines, 1)?);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(count_calories::<u64>(&puzzle_lines, 3)?, 45000);
        assert_eq!(count_calories::<u64>(&puzzle_lines, 3)?, count_calories_orig(&puzzle_lines, 3)?);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(count_calories::<u64>(&puzzle_lines, 3)?, 203420);
        assert_eq!(count_calories::<u64>(&puzzle_lines, 3)?, count_calories_orig(&puzzle_lines, 3)?);
        Ok(())
    }
}
