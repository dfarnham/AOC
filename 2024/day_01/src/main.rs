use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

fn get_columns(puzzle_lines: &[String]) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let mut column1 = vec![];
    let mut column2 = vec![];

    for line in puzzle_lines.iter().filter(|line| !line.is_empty()) {
        let fields = trim_split_ws::<usize>(line)?;
        column1.push(fields[0]);
        column2.push(fields[1]);
    }
    Ok((column1, column2))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mut column1, mut column2) = get_columns(puzzle_lines)?;
    column1.sort_unstable();
    column2.sort_unstable();

    Ok(column1
        .iter()
        .zip(column2)
        .map(|(a, b)| a.abs_diff(b))
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (column1, column2) = get_columns(puzzle_lines)?;
    let column2_counts = column2.into_iter().collect::<Counter<usize>>();

    Ok(column1.into_iter().map(|n| n * column2_counts[&n]).sum())
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
        assert_eq!(part1(&puzzle_lines)?, 11);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 765748);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 31);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 27732508);
        Ok(())
    }
}
