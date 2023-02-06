use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Vec<usize> {
    const RADIX: u32 = 10;
    data[0].chars().map(|c| c.to_digit(RADIX).unwrap() as usize).collect()
}

fn permute(next_value: &mut [usize], start: usize, iterations: usize, high: usize) {
    let mut current = start;
    for _ in 0..iterations {
        // the 3 values being removed
        let r1 = next_value[current];
        let r2 = next_value[r1];
        let r3 = next_value[r2];

        let removed = [r1, r2, r3];

        // starting at one less than the current value,
        // repeat subtracting while the value is in removed
        let mut next_lowest = current - 1;
        while removed.contains(&next_lowest) {
            next_lowest -= 1;
        }

        // if the next_lowest was set to zero, re-search starting at the high value,
        // otherwise keep the next_lowest found above
        let insert_after = match next_lowest {
            0 => {
                next_lowest = high;
                while removed.contains(&next_lowest) {
                    next_lowest -= 1;
                }
                next_lowest
            }
            _ => next_lowest,
        };

        // set the next current value and update next_value pointers
        let prev_current = current;
        current = next_value[r3];
        next_value[prev_current] = current;
        next_value[r3] = next_value[insert_after];
        next_value[insert_after] = r1;
    }
}

fn solution1(data: &[String]) -> String {
    let list = get_data(data);

    let mut next_value = vec![0; list.len() + 1];
    for i in 0..list.len() - 1 {
        next_value[list[i]] = list[i + 1];
    }
    next_value[list[list.len() - 1]] = list[0];

    permute(&mut next_value, list[0], 100, list.len());

    let mut s = String::new();
    let mut current = 1;
    for _ in 0..list.len() - 1 {
        current = next_value[current];
        s += &current.to_string();
    }
    s
}

fn solution2(data: &[String]) -> usize {
    let list = get_data(data);
    let n = 1000000;

    let mut next_value = vec![0; n + 1];
    for i in 0..list.len() - 1 {
        next_value[list[i]] = list[i + 1];
    }
    next_value[list[list.len() - 1]] = list.len() + 1;

    for i in list.len() + 1..next_value.len() - 1 {
        next_value[i] = i + 1;
    }
    next_value[n] = list[0];

    permute(&mut next_value, list[0], 10*n, n);

    next_value[1] * next_value[next_value[1]]
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
        assert_eq!("67384529", solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!("47598263", solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(149245887792, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(248009574232, solution2(&data));
    }
}
