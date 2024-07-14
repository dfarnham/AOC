use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn get_data(puzzle_lines: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(puzzle_lines[0].chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
}

fn fft(digits: &[u8]) -> Vec<u8> {
    let mut new_digits = vec![];

    for position in 1..=digits.len() {
        let mut first = true;
        let mut index = 0;
        let mut ctr = 0;
        let mut sum = 0;
        while ctr < digits.len() {
            // PATTERN[index] is repeated `position` times (skipping once)
            for _ in 0..position {
                if first {
                    first = false;
                    continue;
                }
                sum += match PATTERN[index] {
                    0 => 0,
                    1 => digits[ctr] as i64,
                    _ => -(digits[ctr] as i64),
                };
                ctr += 1;
                if ctr == digits.len() {
                    break;
                }
            }
            index = (index + 1) % PATTERN.len();
        }
        new_digits.push((sum.abs() % 10) as u8);
    }
    new_digits
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut digits = get_data(puzzle_lines)?;

    // 100 phases
    for _ in 1..=100 {
        digits = fft(&digits);
    }

    // convert the first 8 digits into a number
    Ok(digits.iter().take(8).fold(0, |acc, d| acc * 10 + *d as usize))
}

fn part2(puzzle_lines: &[String], repeat: usize) -> Result<usize, Box<dyn Error>> {
    let mut digits = get_data(puzzle_lines)?;
    let offset = digits.iter().take(7).fold(0, |acc, d| acc * 10 + *d as usize);

    // this is a bit of a fudge and depends on the offset being large enough that the pattern is of form:
    //   00000... [offset] 11111...
    // which implies we're only summing the digits from:
    //   offset..digits.len(), offset+1..digits.len(), offset+2..digits.len(), ...
    assert!(offset > digits.len() / 2);

    // partial block from offset..digits.len()
    let mut new_digits = digits[offset % digits.len()..digits.len()].to_vec();

    // pad with remaining full blocks of digits
    for _ in 0..(repeat*digits.len() - offset) / digits.len() {
        new_digits.extend(&digits);
    }
    digits = new_digits;

    // perform 100 phases with a known pattern (just a sum because offset is large)
    for _ in 0..100 {
        let mut digits_sum = digits.iter().fold(0, |acc, d| acc + *d as usize);
        let mut new_digits = vec![];
        for d in digits {
            new_digits.push((digits_sum % 10) as u8);
            // every iteration will add one more leading zero, thus each successive sum
            // decreases by the previous digit
            //
            // The PATTERN is progressing as follows
            // 00001111
            // 00000111
            // 00000011
            // ...
            digits_sum -= d as usize;
        }
        digits = new_digits;
    }

    // convert the first 8 digits into a number
    Ok(digits.iter().take(8).fold(0, |acc, d| acc * 10 + *d as usize))
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
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines, 10000)?)?;

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
        assert_eq!(part1(&puzzle_lines)?, 23845678);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part1(&puzzle_lines)?, 24176176);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part1(&puzzle_lines)?, 73745418);
        Ok(())
    }

    #[test]
    fn part1_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part1(&puzzle_lines)?, 52432133);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 44098263);
        Ok(())
    }

    #[test]
    fn part2_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
        assert_eq!(part2(&puzzle_lines, 10000)?, 84462026);
        Ok(())
    }

    #[test]
    fn part2_example6() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example6");
        assert_eq!(part2(&puzzle_lines, 10000)?, 78725270);
        Ok(())
    }

    #[test]
    fn part2_example7() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example7");
        assert_eq!(part2(&puzzle_lines, 10000)?, 53553731);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines, 10000)?, 12482168);
        Ok(())
    }
}
