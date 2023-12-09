use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

/// num_wins: returns a list, number of mathching winners on each card
fn num_wins(puzzle_lines: &[String]) -> Vec<usize> {
    //        winning numbers        other numbers
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // ...
    puzzle_lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|halves| {
            (
                halves.0.split_whitespace().collect::<HashSet<_>>(),
                halves.1.split_whitespace().collect::<HashSet<_>>(),
            )
        })
        .map(|(winners, others)| winners.intersection(&others).count())
        .collect()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(num_wins(puzzle_lines)
        .into_iter()
        .filter(|n| *n > 0)
        .map(|n| 1 << (n - 1))
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(num_wins(puzzle_lines)
        .into_iter()
        .fold((0, 0, HashMap::new()), |(acc, i, mut counter), n| {
            let count = counter.remove(&i).unwrap_or(0) + 1;
            for j in i + 1..i + 1 + n {
                *counter.entry(j).or_insert(0) += count;
            }
            (acc + n * count + 1, i + 1, counter)
        })
        .0)
}

#[allow(dead_code)]
fn part2_orig(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut counter = HashMap::new();
    for (i, n) in num_wins(puzzle_lines).iter().enumerate() {
        *counter.entry(i).or_insert(0) += 1;
        for j in i + 1..i + 1 + n {
            *counter.entry(j).or_insert(0) += counter[&i];
        }
    }
    Ok(counter.values().sum())
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
        assert_eq!(part1(&puzzle_lines)?, 13);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 23678);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 30);
        assert_eq!(part2_orig(&puzzle_lines)?, 30);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 15455663);
        assert_eq!(part2_orig(&puzzle_lines)?, 15455663);
        Ok(())
    }
}
