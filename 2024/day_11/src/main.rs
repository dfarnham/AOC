use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

fn solve(puzzle_lines: &[String], iterations: usize) -> Result<usize, Box<dyn Error>> {
    let mut stones = trim_split_ws::<usize>(&puzzle_lines[0])?
        .into_iter()
        .collect::<Counter<_>>();

    let transform = |engraved_value: usize| -> Result<Vec<usize>, Box<dyn Error>> {
        let ev = engraved_value.to_string();

        Ok(if engraved_value == 0 {
            // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1
            vec![1]
        } else if ev.len() % 2 == 0 {
            // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
            // The left half of the digits are engraved on the new left stone, and the right half of the digits are
            // engraved on the new right stone. (The new numbers don't keep extra leading zeroes)
            let m = ev.len() / 2;
            vec![ev[..m].parse::<usize>()?, ev[m..].parse::<usize>()?]
        } else {
            // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024
            vec![engraved_value * 2024]
        })
    };

    for _ in 0..iterations {
        let mut output = Counter::<_>::new();
        for (s, c) in stones {
            for t in transform(s)? {
                output[&t] += c;
            }
        }
        stones = output;
    }
    Ok(stones.values().sum())
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

    let n = solve(&puzzle_lines, 25)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = solve(&puzzle_lines, 75)?;
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
        assert_eq!(solve(&puzzle_lines, 0)?, 5);
        assert_eq!(solve(&puzzle_lines, 1)?, 7);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, 0)?, 2);
        assert_eq!(solve(&puzzle_lines, 1)?, 3);
        assert_eq!(solve(&puzzle_lines, 2)?, 4);
        assert_eq!(solve(&puzzle_lines, 3)?, 5);
        assert_eq!(solve(&puzzle_lines, 4)?, 9);
        assert_eq!(solve(&puzzle_lines, 5)?, 13);
        assert_eq!(solve(&puzzle_lines, 6)?, 22);
        assert_eq!(solve(&puzzle_lines, 25)?, 55312);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, 25)?, 220999);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, 75)?, 261936432123724);
        Ok(())
    }
}
