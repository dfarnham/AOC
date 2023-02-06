use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

// Given an input file containing key/value pairs separated by whitespace
// Return a map of summed values for each key and a "depth" calculation
fn get_move_data(filename: Option<PathBuf>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut move_data = HashMap::new();
    let mut aim = 0;
    let mut depth = 0;

    for line in read_trimmed_data_lines::<String>(filename.as_ref())? {
        let fields = line.split_whitespace().collect::<Vec<&str>>();
        assert!(
            fields.len() == 2,
            "Expected 2 fields have {}: {:?}",
            fields.len(),
            fields
        );

        let units = move_data.entry(fields[0].into()).or_insert(0);
        let value = fields[1].parse::<i32>()?;
        *units += value;

        match fields[0] {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => depth += aim * value,
            _ => panic!("Unknown command {}", fields[0]),
        }
    }
    move_data.insert("depth".into(), depth);
    Ok(move_data)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    let file = args.get_one::<std::path::PathBuf>("FILE");
    let move_data = get_move_data(file.cloned())?;
    let forward = move_data.get("forward").ok_or("missing key \"forward\"")?;
    let up = move_data.get("up").ok_or("missing key \"up\"")?;
    let down = move_data.get("down").ok_or("missing key \"down\"")?;
    let depth = move_data.get("depth").ok_or("missing key \"depth\"")?;
    writeln!(stdout, "Answer Part 1 = {}", forward * (down - up))?;
    writeln!(stdout, "Answer Part 2 = {}", forward * depth)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1(filename: &str) -> i32 {
        let file = Some(std::path::PathBuf::from(filename));
        let data = get_move_data(file).unwrap();
        data.get("forward").unwrap() * (data.get("down").unwrap() - data.get("up").unwrap())
    }

    fn part2(filename: &str) -> i32 {
        let file = Some(std::path::PathBuf::from(filename));
        let data = get_move_data(file).unwrap();
        data.get("forward").unwrap() * data.get("depth").unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("input-example"), 150);
    }

    #[test]
    fn part1_actual() {
        assert_eq!(part1("input-actual"), 1938402);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("input-example"), 900);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2("input-actual"), 1947878632);
    }
}
