use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

fn solution(puzzle_lines: &[String], p1: bool) -> Result<i64, Box<dyn Error>> {
    let mut extrapolated = vec![];
    for line in puzzle_lines {
        let mut seq = trim_split_ws::<i64>(line)?;
        if seq.is_empty() {
            continue;
        }

        let mut sequences = vec![seq.clone()];
        while !seq.iter().all(|n| *n == 0) {
            seq = seq.windows(2).map(|w| w[1] - w[0]).collect();
            sequences.push(seq.clone());
        }

        let value = sequences.iter().rev().fold(0, |acc, seq| match p1 {
            true => acc + seq.last().unwrap(),
            false => seq.first().unwrap() - acc,
        });
        extrapolated.push(value);
    }
    Ok(extrapolated.iter().sum())
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    solution(puzzle_lines, true)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    solution(puzzle_lines, false)
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
        assert_eq!(part1(&puzzle_lines)?, 114);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 2008960228);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 2);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 1097);
        Ok(())
    }
}
