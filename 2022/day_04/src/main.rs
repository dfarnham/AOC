use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};
use std::ops::RangeInclusive;

fn ranges(puzzle_lines: &[String]) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    puzzle_lines
        .iter()
        .map(|line| trim_split_on::<String>(line, ',').unwrap())
        .map(|start_end| {
            (
                trim_split_on::<usize>(&start_end[0], '-').unwrap(),
                trim_split_on::<usize>(&start_end[1], '-').unwrap(),
            )
        })
        .map(|p| (p.0[0]..=p.0[1], p.1[0]..=p.1[1]))
        .collect()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(ranges(puzzle_lines)
        .iter()
        .filter(|r| {
            r.1.contains(r.0.start()) && r.1.contains(r.0.end())
                || r.0.contains(r.1.start()) && r.0.contains(r.1.end())
        })
        .count())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(ranges(puzzle_lines)
        .iter()
        .filter(|r| {
            r.1.contains(r.0.start())
                || r.1.contains(r.0.end())
                || r.0.contains(r.1.start())
                || r.0.contains(r.1.end())
        })
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

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

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
        assert_eq!(part1(&puzzle_lines)?, 2);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 487);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 4);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 849);
        Ok(())
    }
}
