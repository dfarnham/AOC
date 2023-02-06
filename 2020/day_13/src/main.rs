use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn solution1(data: &[String]) -> usize {
    let minutes = data[0].parse::<usize>().expect("can't parse");
    match trim_split_on::<String>(&data[1], ',')
        .unwrap()
        .iter()
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().expect("can't parse"))
        .map(|b| (b, b - minutes % b))
        .min_by(|x, y| x.1.cmp(&y.1))
    {
        Some(bus) => bus.0 * bus.1,
        None => 0,
    }
}

fn solution2(data: &[String]) -> usize {
    let mut timestamp = 0;
    let mut inc = 0;
    trim_split_on::<String>(&data[1], ',')
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse::<usize>().expect("can't parse")))
        .for_each(|(i, bus)| {
            if i == 0 {
                inc = bus
            } else {
                while (timestamp + i) % bus != 0 {
                    timestamp += inc;
                }
                inc *= bus;
            }
        });
    timestamp
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
        assert_eq!(295, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(8063, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(1068781, solution2(&data));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(3417, solution2(&["".to_string(), "17,x,13,19".to_string()]));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(754018, solution2(&["".to_string(), "67,7,59,61".to_string()]));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(779210, solution2(&["".to_string(), "67,x,7,59,61".to_string()]));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(1261476, solution2(&["".to_string(), "67,7,x,59,61".to_string()]));
    }

    #[test]
    fn part2_example6() {
        assert_eq!(1202161486, solution2(&["".to_string(), "1789,37,47,1889".to_string()]));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(775230782877242, solution2(&data));
    }
}
