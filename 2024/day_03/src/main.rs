use general::{get_args, read_trimmed_data_lines, reset_sigpipe, split_by_delimiter};
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = puzzle_lines.join("");

    // find all "mul(number,number)" instructions and sum the products
    Ok(re.captures_iter(&input).fold(0, |acc, cap| {
        acc + cap.get(1).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap()
            * cap.get(2).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap()
    }))
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let input = puzzle_lines.join("");

    // split the input on "don't()"
    let donts = split_by_delimiter(&input, "don't()");

    // everything up to the first don't() is enabled by default
    let mut enabled = vec![];
    enabled.push(donts[0].clone());

    // split the remaining "don't()" segment on "do()" and skip the first split within each of those segments
    enabled.extend(donts.into_iter().skip(1).flat_map(|dont| {
        split_by_delimiter(&dont, "do()")
            .into_iter()
            .skip(1)
            .collect::<Vec<_>>()
    }));

    // part1 handles finding and calculating the mul() instructions
    part1(&enabled)
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

    let n = part1(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = part2(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 2 = {n}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file = std::path::PathBuf::from(filename);
        Ok(read_trimmed_data_lines(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part1(&puzzle_lines)?, 161);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 160672468);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part2(&puzzle_lines)?, 48);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 84893551);
        Ok(())
    }
}
