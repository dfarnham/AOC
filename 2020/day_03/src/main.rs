use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn solution1(data: &[String], x: usize, y: usize) -> usize {
    let mut col = 0;
    data.iter()
        .skip(y)
        .step_by(y)
        .filter(|row| {
            col = (col + x) % row.len();
            &row[col..=col] == "#"
        })
        .count()
}

fn solution2(data: &[String]) -> usize {
    [
        solution1(data, 1, 1),
        solution1(data, 3, 1),
        solution1(data, 5, 1),
        solution1(data, 7, 1),
        solution1(data, 1, 2),
    ]
    .iter()
    .product()
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

    writeln!(stdout, "Answer Part 1 = {}", solution1(&puzzle_lines, 3, 1))?;
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
        assert_eq!(7, solution1(&data, 3, 1));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(211, solution1(&data, 3, 1));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(336, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(3584591857, solution2(&data));
    }
}
