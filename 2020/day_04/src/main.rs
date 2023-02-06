use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn is_valid1(passport_data: &HashSet<String>) -> bool {
    !passport_data.is_empty() && passport_data.len() == 8
        || (passport_data.len() == 7 && !passport_data.contains("cid"))
}

fn is_valid2(passport_data: &HashMap<String, String>) -> bool {
    let re_hcl: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let re_pid: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let re_ecl: Regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();

    if !passport_data.is_empty() && passport_data.len() == 8
        || (passport_data.len() == 7 && !passport_data.contains_key("cid"))
    {
        let byr = passport_data.get("byr").unwrap().parse::<u32>().unwrap();
        let byr = (1920..=2002).contains(&byr);

        let iyr = passport_data.get("iyr").unwrap().parse::<u32>().unwrap();
        let iyr = (2010..=2020).contains(&iyr);

        let eyr = passport_data.get("eyr").unwrap().parse::<u32>().unwrap();
        let eyr = (2020..=2030).contains(&eyr);

        let hgt = passport_data.get("hgt").unwrap();
        let hgt = match hgt.ends_with("in") {
            true => (59..=76).contains(&hgt.replace("in", "").parse::<u32>().unwrap()),
            false => match hgt.ends_with("cm") {
                true => (150..=193).contains(&hgt.replace("cm", "").parse::<u32>().unwrap()),
                false => false,
            },
        };

        let hcl = re_hcl.is_match(passport_data.get("hcl").unwrap());
        let ecl = re_ecl.is_match(passport_data.get("ecl").unwrap());
        let pid = re_pid.is_match(passport_data.get("pid").unwrap());

        byr && iyr && eyr && hgt && hcl && ecl && pid
    } else {
        false
    }
}

fn solution1(data: &[String]) -> usize {
    let mut passport_data = HashSet::new();
    let mut valid = 0;

    for line in data {
        if line.is_empty() {
            if is_valid1(&passport_data) {
                valid += 1;
            }
            passport_data.clear();
        }

        for field in line.split_whitespace() {
            let kv = trim_split_on::<String>(field, ':').unwrap();
            passport_data.insert(kv[0].clone());
        }
    }

    if is_valid1(&passport_data) {
        valid += 1;
    }
    valid
}

fn solution2(data: &[String]) -> usize {
    let mut passport_data = HashMap::new();
    let mut valid = 0;

    for line in data {
        if line.is_empty() {
            if is_valid2(&passport_data) {
                valid += 1;
            }
            passport_data.clear();
        }

        for field in line.split_whitespace() {
            let kv = trim_split_on::<String>(field, ':').unwrap();
            passport_data.insert(kv[0].clone(), kv[1].clone());
        }
    }

    if is_valid2(&passport_data) {
        valid += 1;
    }
    valid
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
        assert_eq!(182, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(2, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(109, solution2(&data));
    }
}
