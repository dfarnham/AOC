use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

const WIN: u64 = 6;
const LOSE: u64 = 0;
const DRAW: u64 = 3;

fn game(puzzle_lines: &[String]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0;
    for line in puzzle_lines.iter() {
        let mut choices = line.chars();
        let (player1, _, player2) = (choices.next(), choices.next(), choices.next());
        total += match player2 {
            Some('X') => match player1 {
                Some('A') => ROCK + DRAW,
                Some('B') => ROCK + LOSE,
                _ => ROCK + WIN,
            },
            Some('Y') => match player1 {
                Some('A') => PAPER + WIN,
                Some('B') => PAPER + DRAW,
                _ => PAPER + LOSE,
            },
            _ => match player1 {
                Some('A') => SCISSORS + LOSE,
                Some('B') => SCISSORS + WIN,
                _ => SCISSORS + DRAW,
            },
        };
    }
    Ok(total)
}

fn part1(puzzle_lines: &[String]) -> Result<u64, Box<dyn Error>> {
    game(puzzle_lines)
}

fn part2(puzzle_lines: &[String]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0;
    for line in puzzle_lines.iter() {
        let mut choices = line.chars();
        let (player1, _, player2) = (choices.next(), choices.next(), choices.next());
        total += match player2 {
            Some('X') => match player1 {
                Some('A') => SCISSORS + LOSE,
                Some('B') => ROCK + LOSE,
                _ => PAPER + LOSE,
            },
            Some('Y') => match player1 {
                Some('A') => ROCK + DRAW,
                Some('B') => PAPER + DRAW,
                _ => SCISSORS + DRAW,
            },
            _ => match player1 {
                Some('A') => PAPER + WIN,
                Some('B') => SCISSORS + WIN,
                _ => ROCK + WIN,
            },
        };
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
        assert_eq!(part1(&puzzle_lines)?, 15);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 11475);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 12);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 16862);
        Ok(())
    }
}
