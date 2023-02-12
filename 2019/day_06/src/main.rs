use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut dependencies = HashMap::new();
    for line in puzzle_lines {
        let planets = trim_split_on::<String>(line, ')')?;
        dependencies.insert(planets[1].to_owned(), planets[0].to_owned());
    }
    Ok(dependencies)
}

fn path_to_planet(src: &str, dst: &str, dependencies: &HashMap<String, String>) -> Vec<String> {
    let mut path = vec![];
    let mut src = src;
    while src != dst {
        let new_src = dependencies.get(src).unwrap_or_else(|| panic!("invalid key: {src}"));
        path.push(new_src.into());
        src = new_src;
    }
    path
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let dependencies = get_data(puzzle_lines)?;
    Ok(dependencies
        .keys()
        .map(|k| path_to_planet(k, "COM", &dependencies).len())
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let dependencies = get_data(puzzle_lines)?;
    let you = path_to_planet("YOU", "COM", &dependencies);
    let san = path_to_planet("SAN", "COM", &dependencies);
    for (i, planet) in san.iter().enumerate() {
        if let Some(index) = you.iter().position(|p| p.as_str() == planet) {
            return Ok(index + i);
        }
    }
    Err("no solution".into())
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines)?)?;

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
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 54);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 344238);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 4);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 436);
        Ok(())
    }
}
