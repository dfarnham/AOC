use general::{get_args, read_data_lines, reset_sigpipe};
//use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Vec<i64> {
    data.iter().map(|s| snafu_to_base10(s)).collect()
}

fn snafu_to_base10(s: &str) -> i64 {
    const RADIX: u32 = 10;

    let mut n = 0;
    let mut pow5 = 1;
    for c in s.chars().rev() {
        n += match c {
            '-' => -pow5,
            '=' => -2 * pow5,
            c => c.to_digit(RADIX).unwrap() as i64 * pow5,
        };
        pow5 *= 5;
    }
    n
}

#[rustfmt::skip]
fn base10_to_snafu(n: i64) -> String {
    let mut n = n;
    let mut s = String::new();

    let powers = ((n as f64).ln() / 5_f64.ln()) as u32;
    let mut pow5 = 5_i64.pow(powers);
    for _ in 0..=powers {
        let digit = n / pow5;
        s += &digit.to_string();
        n -= digit * pow5;
        pow5 /= 5;
    }

    let base5_rs = s.chars().rev().collect::<String>();
    s.clear();
    let mut carry = false;
    for c in base5_rs.chars() {
        s += match c {
            '0' => match carry {
                true => { carry = false; "1" }
                false => "0",
            },
            '1' => match carry {
                true => { carry = false; "2" }
                false => "1",
            },
            '2' => match carry {
                true => "=",
                false => "2",
            },
            '3' => match carry {
                true => "-",
                false => { carry = true; "=" }
            },
            '4' => match carry {
                true => "0",
                false => { carry = true; "-" }
            },
            _ => panic!("oops"),
        }
    }
    if s.ends_with('-') || s.ends_with('=') {
        s += "1";
    }
    s.chars().rev().collect::<String>()
}

// this is much cleaner
// https://github.com/hyper-neutrino/advent-of-code/blob/main/2022/day25.py
fn part1(puzzle_lines: &[String]) -> Result<String, Box<dyn Error>> {
    let mut total: i64 = 0;
    for line in puzzle_lines {
        let mut coef = 1;
        for c in line.chars().rev() {
            total += match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("oops"),
            } * coef;
            coef *= 5;
        }
    }

    let mut output = String::new();
    while total > 0 {
        let rem = total % 5;
        total /= 5;

        output = match rem {
            0 | 1 | 2 => rem.to_string() + &output,
            3 => "=".to_string() + &output,
            _ => "-".to_string() + &output,
        };
        if rem > 2 {
            total += 1;
        }
    }
    Ok(output)
}

#[allow(dead_code)]
fn part1_orig(puzzle_lines: &[String]) -> Result<String, Box<dyn Error>> {
    let nums = get_data(puzzle_lines);
    Ok(base10_to_snafu(nums.iter().sum()))
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    //writeln!(stdout, "Answer Part 1 = {}", part1_orig(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    //writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

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
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, "2=-1=0");
        assert_eq!(part1_orig(&puzzle_lines)?, "2=-1=0");
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, "121=2=1==0=10=2-20=2");
        assert_eq!(part1_orig(&puzzle_lines)?, "121=2=1==0=10=2-20=2");
        Ok(())
    }

    const B10: [i64; 26] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 20, 31, 32, 37, 107, 198, 201, 353, 906, 1257, 1747,
        2022, 12345, 314159265,
    ];

    const SNAFU: [&str; 26] = [
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
        "21",
        "1=0",
        "1-0",
        "111",
        "112",
        "122",
        "1-12",
        "2=0=",
        "2=01",
        "1=-1=",
        "12111",
        "20012",
        "1=-0-2",
        "1=11-2",
        "1-0---0",
        "1121-1110-1=0",
    ];

    #[test]
    fn snafu_test() -> Result<(), Box<dyn Error>> {
        for i in 0..B10.len() {
            assert_eq!(base10_to_snafu(B10[i]), SNAFU[i]);
            assert_eq!(snafu_to_base10(SNAFU[i]), B10[i]);
        }
        Ok(())
    }
}
