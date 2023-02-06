use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn find_marker(puzzle: &str, window: usize) -> Result<usize, Box<dyn Error>> {
    for (i, w) in puzzle
        .chars()
        .collect::<Vec<_>>()
        .windows(window)
        .enumerate()
    {
        if w.iter().copied().collect::<HashSet<_>>().len() == window {
            return Ok(window + i);
        }
    }
    Err(Box::from("no solution"))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    find_marker(&puzzle_lines[0], 4)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    find_marker(&puzzle_lines[0], 14)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

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
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 7);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part1(&puzzle_lines)?, 5);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part1(&puzzle_lines)?, 6);
        Ok(())
    }

    #[test]
    fn part1_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part1(&puzzle_lines)?, 10);
        Ok(())
    }

    #[test]
    fn part1_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
        assert_eq!(part1(&puzzle_lines)?, 11);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1658);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 19);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part2(&puzzle_lines)?, 23);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part2(&puzzle_lines)?, 23);
        Ok(())
    }

    #[test]
    fn part2_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part2(&puzzle_lines)?, 29);
        Ok(())
    }

    #[test]
    fn part2_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
        assert_eq!(part2(&puzzle_lines)?, 26);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2260);
        Ok(())
    }
}
