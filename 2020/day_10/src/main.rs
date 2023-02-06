use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashMap;

fn solution1(data: &[usize]) -> usize {
    let mut sorted = data.to_vec();
    sorted.sort_unstable();
    let mut target = 0;
    let mut diff1 = 0;
    let mut diff3 = 1;
    for n in sorted {
        if n - target == 1 {
            diff1 += 1;
        } else if n - target == 3 {
            diff3 += 1;
        }
        target = n;
    }
    diff1 * diff3
}

fn solution2(data: &[usize]) -> usize {
    let mut sorted = data.to_vec();
    sorted.push(0);
    sorted.sort_unstable();
    sorted.push(sorted.last().unwrap() + 3);
    calc(&mut HashMap::new(), &sorted, 0)
}

fn calc(memoize: &mut HashMap<usize, usize>, sorted: &[usize], i: usize) -> usize {
    match i == sorted.len() - 1 {
        true => 1,
        false => match memoize.get(&i) {
            Some(n) => *n,
            None => {
                let sum = (i + 1..sorted.len())
                    .filter(|j| sorted[*j] - sorted[i] <= 3)
                    .map(|j| calc(memoize, sorted, j))
                    .sum();
                memoize.insert(i, sum);
                sum
            }
        }
    }
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

    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&puzzle_lines))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<usize> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<usize>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(35, solution1(&data));
    }

    #[test]
    fn part1_example2() {
        let data = get_data("input-example2");
        assert_eq!(220, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(1690, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(8, solution2(&data));
    }

    #[test]
    fn part2_example2() {
        let data = get_data("input-example2");
        assert_eq!(19208, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(5289227976704, solution2(&data));
    }
}
