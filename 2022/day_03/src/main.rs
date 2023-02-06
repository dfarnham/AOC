use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn value(set: &HashSet<char>) -> usize {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    set.iter()
        .map(|c| match c.is_lowercase() {
            true => 1 + *c as usize - 'a' as usize,
            false => 27 + *c as usize - 'A' as usize,
        })
        .sum()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(puzzle_lines
        .iter()
        .map(|line| {
            let set1: HashSet<char> = line.chars().take(line.len() / 2).collect();
            let set2: HashSet<char> = line.chars().skip(line.len() / 2).collect();
            value(&set1.intersection(&set2).copied().collect())
        })
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut total = 0;
    let mut set = HashSet::new();
    for (i, line) in puzzle_lines.iter().enumerate() {
        let line_set: HashSet<char> = line.chars().collect();
        set = match set.is_empty() {
            true => line_set.clone(),
            false => set.intersection(&line_set).copied().collect(),
        };
        if (i + 1) % 3 == 0 {
            total += value(&set);
            set.clear()
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
        assert_eq!(part1(&puzzle_lines)?, 157);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 7742);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 70);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2276);
        Ok(())
    }
}
