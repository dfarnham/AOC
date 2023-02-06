use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;

// Return 2 numbers from the input which sum to target
fn twosum(data: &[i32], target: i32) -> Option<(i32, i32)> {
    assert!(data.len() > 1, "data length less than 2");

    let mut pool = HashSet::new();
    for n in data {
        match target - n {
            m if pool.contains(&m) => return Some((*n, m)),
            _ => pool.insert(*n),
        };
    }
    None
}

// Return 3 numbers from the input which sum to target
fn threesum(data: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    assert!(data.len() > 2, "data length less than 3");

    for (i, n) in data.iter().enumerate() {
        // others set to all data values except data[i]
        let mut others = data.to_vec();
        others.swap_remove(i);
        if let Some((a, b)) = twosum(&others, target - n) {
            return Some((*n, a, b));
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

    let (a, b) = twosum(&puzzle_lines, 2020).ok_or("no solution")?;
    //writeln!(stdout, "{a}, {b}")?;
    writeln!(stdout, "Answer Part 1 = {}", a * b)?;
    let (a, b, c) = threesum(&puzzle_lines, 2020).ok_or("no solution")?;
    //writeln!(stdout, "{a}, {b}, {c}")?;
    writeln!(stdout, "Answer Part 2 = {}", a * b * c)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<i32> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    #[should_panic]
    fn empty_array() {
        let data = vec![];
        let target = 1;
        twosum(&data, target);
    }

    #[test]
    #[should_panic]
    fn array_too_small() {
        let data = vec![199];
        let target = 1;
        twosum(&data, target);
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        let target = 2020;
        let (a, b) = twosum(&data, target).unwrap();
        assert_eq!(a * b, 514579);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let target = 2020;
        let (a, b, c) = threesum(&data, target).unwrap();
        assert_eq!(a * b * c, 241861950);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let target = 2020;
        let (a, b) = twosum(&data, target).unwrap();
        assert_eq!(a * b, 987339);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let target = 2020;
        let (a, b, c) = threesum(&data, target).unwrap();
        assert_eq!(a * b * c, 259521570);
    }
}
