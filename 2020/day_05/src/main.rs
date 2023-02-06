use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;

fn get_seat_ids(data: &[String]) -> Vec<usize> {
    let mut seat_ids = vec![];
    for line in data {
        let mut lower = 0;
        let mut row = 127;
        for ch in line.chars().take(7) {
            match ch == 'F' {
                true => row = row - (row + 1 - lower) / 2,
                false => lower = lower + (row - lower) / 2 + 1,
            };
        }

        let mut lower = 0;
        let mut seat = 7;
        for ch in line.chars().skip(7) {
            match ch == 'L' {
                true => seat = seat - (seat + 1 - lower) / 2,
                false => lower = lower + (seat - lower) / 2 + 1,
            };
        }
        seat_ids.push(row * 8 + seat);
    }
    seat_ids
}

fn solution1(data: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(*get_seat_ids(data).iter().max().ok_or("max() failure")?)
}

fn solution2(data: &[String]) -> Result<usize, Box<dyn Error>> {
    let seat_ids = HashSet::<usize>::from_iter(get_seat_ids(data).iter().cloned());
    let min = *seat_ids.iter().min().ok_or("min() failure")?;
    let max = *seat_ids.iter().max().ok_or("max() failure")?;
    for seat_id in min..max {
        if !seat_ids.contains(&seat_id) {
            return Ok(seat_id);
        }
    }
    Err("oops".into())
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

    writeln!(stdout, "Answer Part 1 = {}", solution1(&puzzle_lines).unwrap())?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&puzzle_lines).unwrap())?;

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
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(820, solution1(&data).unwrap());
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(858, solution1(&data).unwrap());
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(557, solution2(&data).unwrap());
    }
}
