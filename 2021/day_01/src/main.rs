use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use num::Num;
use std::error::Error;
use std::io::{self, Write};

// Given an input array:
// Count the number of times the sum of measurements in a provided sliding window increases
fn count_window_increase<'a, T>(array: &'a [T], window: usize) -> usize
where
    T: Num + std::cmp::PartialOrd + std::iter::Sum<&'a T>,
{
    assert!(window > 0, "Window must be > 0");
    assert!(
        array.len() > window,
        "Array length: {} must be greater than the window size: {}",
        array.len(),
        window
    );

    (0..(array.len() - window))
        .filter(|&i| array[i..(i + window)].iter().sum::<T>() < array[(i + 1)..=(i + window)].iter().sum::<T>())
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_trimmed_data_lines::<u32>(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", count_window_increase(&puzzle_lines, 1))?;
    writeln!(stdout, "Answer Part 2 = {}", count_window_increase(&puzzle_lines, 3))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<u32> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<u32>(Some(&file)).unwrap()
    }

    #[test]
    #[should_panic]
    fn empty_array() {
        let measurements = Vec::<i32>::new();
        let window = 1;
        count_window_increase(&measurements, window);
    }

    #[test]
    #[should_panic]
    fn array_too_small() {
        let measurements = vec![199];
        let window = 1;
        count_window_increase(&measurements, window);
    }

    #[test]
    #[should_panic]
    fn invalid_window() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 0;
        count_window_increase(&measurements, window);
    }

    #[test]
    fn part1_example() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 1;
        assert_eq!(count_window_increase(&measurements, window), 7);

        let measurements = get_data("input-example");
        assert_eq!(count_window_increase(&measurements, window), 7);
    }

    #[test]
    fn part2_example() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 3;
        assert_eq!(count_window_increase(&measurements, window), 5);

        let measurements = get_data("input-example");
        assert_eq!(count_window_increase(&measurements, window), 5);
    }

    #[test]
    fn part1_actual() {
        let measurements = get_data("input-actual");
        let window = 1;
        assert_eq!(count_window_increase(&measurements, window), 1233);
    }

    #[test]
    fn part2_actual() {
        let measurements = get_data("input-actual");
        let window = 3;
        assert_eq!(count_window_increase(&measurements, window), 1275);
    }
}
