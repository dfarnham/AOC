use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

fn win_count(time: usize, distance: usize) -> usize {
    (1..(time - 1))
        .filter(|speed| speed * (time - speed) > distance)
        .count()
}

#[rustfmt::skip]
fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let times = trim_split_ws(puzzle_lines[0].split_once(':').unwrap().1)?;
    let distances = trim_split_ws(puzzle_lines[1].split_once(':').unwrap().1)?;

    Ok(times.iter().zip(distances.iter()).map(|(t, d)| win_count(*t, *d)).product())
}

#[rustfmt::skip]
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let t = puzzle_lines[0].split_once(':').unwrap().1.replace(' ', "").parse::<usize>()?;
    let d = puzzle_lines[1].split_once(':').unwrap().1.replace(' ', "").parse::<usize>()?;

    Ok(win_count(t, d))
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
        assert_eq!(part1(&puzzle_lines)?, 288);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 2344708);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 71503);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 30125202);
        Ok(())
    }
}
