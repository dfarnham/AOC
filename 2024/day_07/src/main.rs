use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Write};

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let mut total = 0;
    for line in puzzle_lines.iter().filter(|line| !line.is_empty()) {
        let numbers = trim_split_ws::<usize>(&line.replace(":", ""))?;

        let mut workq = VecDeque::new();
        workq.push_back((numbers[1], 2)); // operand1, index of operand2
        while let Some((n, i)) = workq.pop_front() {
            let mut evaluations = vec![n + numbers[i], n * numbers[i]];
            if part2 {
                evaluations.push(format!("{n}{}", numbers[i]).parse::<usize>()?);
            }

            if i < numbers.len() - 1 {
                for x in evaluations {
                    workq.push_back((x, i + 1));
                }
            } else if evaluations.contains(&numbers[0]) {
                total += numbers[0];
                break;
            }
        }
    }
    Ok(total)
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
        assert_eq!(solve(&puzzle_lines, false)?, 3749);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 1582598718861);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 11387);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 165278151522644);
        Ok(())
    }
}
