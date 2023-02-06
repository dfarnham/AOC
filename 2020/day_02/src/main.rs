use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn solution1(data: &[String]) -> usize {
    // example line: 1-3 a: abcde
    //   parts[0] = "1-3"
    //   parts[1] = "a:"
    //   parts[2] = "abcde"
    data.iter()
        .filter(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            // min and max number of times the given letter must appear for the password to be valid
            let min_max = trim_split_on::<usize>(parts[0], '-').unwrap();
            let min = min_max[0];
            let max = min_max[1];

            // the password char that must meet min/max occurrence policy
            let pwchar = &parts[1][0..1];

            // the password
            let password = parts[2];

            (min..=max).contains(&password.matches(pwchar).count())
        })
        .count()
}

fn solution2(data: &[String]) -> usize {
    // example line: 1-3 a: abcde
    //   parts[0] = "1-3"
    //   parts[1] = "a:"
    //   parts[2] = "abcde"
    data.iter()
        .filter(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            // positions in the password to be checked
            let positions = trim_split_on::<usize>(parts[0], '-').unwrap();
            let pos0 = positions[0] - 1;
            let pos1 = positions[1] - 1;

            // the password char
            let pwchar = &parts[1][0..1];

            // the password
            let password = parts[2];

            (password[pos0..=pos0] == *pwchar && password[pos1..=pos1] != *pwchar)
                || (password[pos0..=pos0] != *pwchar && password[pos1..=pos1] == *pwchar)
        })
        .count()
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

    writeln!(stdout, "Answer Part 1 = {}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&puzzle_lines))?;

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
        assert_eq!(2, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(398, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(1, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(562, solution2(&data));
    }
}
