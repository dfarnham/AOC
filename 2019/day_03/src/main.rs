use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

const U: (i64, i64) = (0, 1);
const D: (i64, i64) = (0, -1);
const R: (i64, i64) = (1, 0);
const L: (i64, i64) = (-1, 0);

#[allow(clippy::type_complexity)]
fn get_data(puzzle_lines: &[String], n: usize) -> Result<Vec<((i64, i64), usize)>, Box<dyn Error>> {
    let mut directions = vec![];
    for m in trim_split_on::<String>(&puzzle_lines[n], ',')? {
        let mut chars = m.chars();
        let v = match chars.next() {
            Some('U') => U,
            Some('D') => D,
            Some('R') => R,
            Some('L') => L,
            _ => panic!("oops"),
        };
        let n = chars.as_str().parse::<usize>()?;
        directions.push((v, n));
    }
    Ok(directions)
}

fn solutions(puzzle_lines: &[String]) -> Result<(usize, usize), Box<dyn Error>> {
    let wire1 = get_data(puzzle_lines, 0)?;
    let wire2 = get_data(puzzle_lines, 1)?;

    // record all of the coordinates and steps for the first wire
    let mut current = (0, 0);
    let mut visited = HashMap::new();
    let mut step1 = 1;

    for ((x, y), n) in wire1 {
        for i in 0..n {
            current.0 += x;
            current.1 += y;
            // don't overwrite a previous (smaller) value
            visited.entry(current).or_insert_with(|| step1 + i as i64);
        }
        step1 += n as i64;
    }

    // look for collisions while stepping through the second wire
    let mut manhattan_crossed = i64::MAX;
    let mut min_steps_crossed = i64::MAX;
    let mut current = (0, 0);
    let mut step2 = 1;
    for ((x, y), n) in wire2 {
        for i in 0..n {
            current.0 += x;
            current.1 += y;
            // wires crossed
            if let Some(step1) = visited.get(&current) {
                manhattan_crossed = manhattan_crossed.min(current.0.abs() + current.1.abs());
                min_steps_crossed = min_steps_crossed.min(step1 + step2 + i as i64);
            }
        }
        step2 += n as i64;
    }
    Ok((manhattan_crossed as usize, min_steps_crossed as usize))
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

    writeln!(stdout, "Answer Part 1 = {:?}", solutions(&puzzle_lines)?.0)?;
    writeln!(stdout, "Answer Part 2 = {:?}", solutions(&puzzle_lines)?.1)?;

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
        assert_eq!(solutions(&puzzle_lines)?.0, 159);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(solutions(&puzzle_lines)?.0, 227);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(solutions(&puzzle_lines)?.1, 610);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(solutions(&puzzle_lines)?.1, 20286);
        Ok(())
    }
}
