use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashMap;

fn solution(data: &[String], turns: usize) -> usize {
    let numbers = trim_split_on::<usize>(&data[0], ',').unwrap();
    let mut spoken_history = HashMap::new();
    for (i, n) in numbers.iter().enumerate() {
        spoken_history.insert(*n, vec![i]);
    }

    let mut last = numbers[numbers.len() - 1];

    for turn in numbers.len()..turns {
        last = match spoken_history.get(&last) {
            Some(seen) => match seen.len() {
                n if n > 1 => seen[n - 1] - seen[n - 2],
                _ => 0,
            },
            None => 0,
        };

        match spoken_history.get_mut(&last) {
            Some(seen) => {
                seen.push(turn);
                if seen.len() == 3 {
                    seen.remove(0);
                }
            }
            None => {
                spoken_history.insert(last, vec![turn]);
            }
        };
    }
    last
}

fn solution1(data: &[String]) -> usize {
    solution(data, 2020)
}

fn solution2(data: &[String]) -> usize {
    solution(data, 30000000)
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

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(436, solution1(&data));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(1, solution1(&["1,3,2".to_string()]));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(10, solution1(&["2,1,3".to_string()]));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(27, solution1(&["1,2,3".to_string()]));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(78, solution1(&["2,3,1".to_string()]));
    }

    #[test]
    fn part1_example6() {
        assert_eq!(438, solution1(&["3,2,1".to_string()]));
    }

    #[test]
    fn part1_example7() {
        assert_eq!(1836, solution1(&["3,1,2".to_string()]));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(1522, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(175594, solution2(&data));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(2578, solution2(&["1,3,2".to_string()]));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(3544142, solution2(&["2,1,3".to_string()]));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(261214, solution2(&["1,2,3".to_string()]));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(6895259, solution2(&["2,3,1".to_string()]));
    }

    #[test]
    fn part2_example6() {
        assert_eq!(18, solution2(&["3,2,1".to_string()]));
    }

    #[test]
    fn part2_example7() {
        assert_eq!(362, solution2(&["3,1,2".to_string()]));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(18234, solution2(&data));
    }
}
