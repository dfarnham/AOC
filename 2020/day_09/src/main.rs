use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;

// Return 2 numbers from the input which sum to target
fn twosum<T>(data: &[T], target: T) -> Option<(T, T)>
where
    T: Ord + Copy + Eq + Hash + Sub<Output = T>,
{
    assert!(data.len() > 1, "data length less than 2");

    let mut pool = HashSet::new();
    for n in data {
        match target - *n {
            m if pool.contains(&m) => return Some((*n, m)),
            _ => pool.insert(*n),
        };
    }
    None
}

fn solution1(data: &[i64], preamble: usize) -> Option<i64> {
    for (i, n) in data.iter().skip(preamble).enumerate() {
        if twosum(&data[i..preamble + i], *n).is_none() {
            return Some(*n);
        }
    }
    None
}

fn solution2(data: &[i64], preamble: usize) -> Option<i64> {
    if let Some(target) = solution1(data, preamble) {
        for i in 0..data.len() - 1 {
            let mut j = 2;
            let mut contiguous = data[i..i + j].to_vec();
            loop {
                match contiguous.iter().sum::<i64>() {
                    sum if sum == target => {
                        return Some(contiguous.iter().min().unwrap() + contiguous.iter().max().unwrap())
                    }
                    sum if sum < target => {
                        contiguous.push(data[i + j]);
                        j += 1
                    }
                    _ => break,
                }
            }
        }
    }
    None
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

    let preamble = match puzzle_lines.len() < 25 {
        true => 5,
        false => 25,
    };
    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines, preamble))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&puzzle_lines, preamble))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<i64> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<i64>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(Some(127), solution1(&data, 5));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(Some(1212510616), solution1(&data, 25));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(Some(62), solution2(&data, 5));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(Some(171265123), solution2(&data, 25));
    }
}
