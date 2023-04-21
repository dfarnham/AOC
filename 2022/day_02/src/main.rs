use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const WIN: usize = 6;
const LOSE: usize = 0;
const DRAW: usize = 3;

fn game_strategy_1(player1: &str, player2: &str) -> usize {
    match player2 {
        "X" => match player1 {
            "A" => ROCK + DRAW,
            "B" => ROCK + LOSE,
            "C" => ROCK + WIN,
            _ => unreachable!(),
        },
        "Y" => match player1 {
            "A" => PAPER + WIN,
            "B" => PAPER + DRAW,
            "C" => PAPER + LOSE,
            _ => unreachable!(),
        },
        "Z" => match player1 {
            "A" => SCISSORS + LOSE,
            "B" => SCISSORS + WIN,
            "C" => SCISSORS + DRAW,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn game_strategy_2(player1: &str, player2: &str) -> usize {
    match player2 {
        "X" => match player1 {
            "A" => SCISSORS + LOSE,
            "B" => ROCK + LOSE,
            "C" => PAPER + LOSE,
            _ => unreachable!(),
        },
        "Y" => match player1 {
            "A" => ROCK + DRAW,
            "B" => PAPER + DRAW,
            "C" => SCISSORS + DRAW,
            _ => unreachable!(),
        },
        "Z" => match player1 {
            "A" => PAPER + WIN,
            "B" => SCISSORS + WIN,
            "C" => ROCK + WIN,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn play_game(puzzle_lines: &[String], game_strategy: &dyn Fn(&str, &str) -> usize) -> Result<usize, Box<dyn Error>> {
    Ok(puzzle_lines.iter().map(|s| game_strategy(&s[0..1], &s[2..3])).sum())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    play_game(puzzle_lines, &game_strategy_1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    play_game(puzzle_lines, &game_strategy_2)
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
