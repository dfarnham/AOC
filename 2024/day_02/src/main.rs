use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

fn level_test(levels: &[i64]) -> bool {
    // The difference between adjacent values must be between 1 and 3
    let adjacent_test = |levels: &[i64]| -> bool { levels.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1]))) };

    // Levels must be sorted (ascending or descending)
    let sorted_test = |levels: &[i64]| -> bool {
        let mut sorted = levels.to_vec();
        sorted.sort_unstable();
        levels == sorted || levels.iter().copied().rev().collect::<Vec<_>>() == sorted
    };

    adjacent_test(levels) && sorted_test(levels)
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let mut passed_count = 0;

    for line in puzzle_lines.iter().filter(|line| !line.is_empty()) {
        let levels = trim_split_ws::<i64>(line)?;
        if level_test(&levels) {
            passed_count += 1;
        } else if part2 {
            // allowed to remove a single element and re-test
            for i in 0..levels.len() {
                let mut skip_a_level = levels.clone();
                skip_a_level.remove(i);
                if level_test(&skip_a_level) {
                    passed_count += 1;
                    break;
                }
            }
        }
    }

    Ok(passed_count)
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

    let n = solve(&puzzle_lines, false)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = solve(&puzzle_lines, true)?;
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
        assert_eq!(solve(&puzzle_lines, false)?, 2);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 220);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 4);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 296);
        Ok(())
    }
}
