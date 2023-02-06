use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use regex::Regex;
use std::collections::HashMap;

fn solution1(data: &[String]) -> u64 {
    let re: Regex = Regex::new(r"^mem\[(\d+)\]\s+=\s+(\d+)").unwrap();
    let mut memory = HashMap::new();
    let mut mask = String::new();

    for line in data {
        if line.starts_with("mask = ") {
            mask = line.replace("mask = ", "");
        } else {
            let caps = re.captures(line).unwrap();
            let address = caps
                .get(1)
                .expect("capture failed")
                .as_str()
                .parse::<usize>()
                .expect("parse failed");
            let mut value = caps
                .get(2)
                .expect("capture failed")
                .as_str()
                .parse::<u64>()
                .expect("parse failed");

            // set the 0, 1 bit positions in "value" as they appear in the "mask"
            for (i, bit) in mask.chars().rev().enumerate().filter(|(_, c)| ['0', '1'].contains(c)) {
                match bit {
                    '0' => value &= !(1 << i),
                    _ => value |= 1 << i,
                };
            }

            // assign: address => value
            memory.insert(address, value);
        }
    }
    memory.values().sum()
}

fn solution2(data: &[String]) -> u64 {
    let re: Regex = Regex::new(r"^mem\[(\d+)\]\s+=\s+(\d+)").unwrap();
    let mut memory = HashMap::new();
    let mut mask = String::new();

    for line in data {
        if line.starts_with("mask = ") {
            mask = line.replace("mask = ", "");
        } else {
            let caps = re.captures(line).unwrap();
            let mut address = caps
                .get(1)
                .expect("capture failed")
                .as_str()
                .parse::<usize>()
                .expect("parse failed");
            let value = caps
                .get(2)
                .expect("capture failed")
                .as_str()
                .parse::<u64>()
                .expect("parse failed");

            let mut floating_pos = vec![];
            // set the bit '1' bit position in "address" as they appear in the "mask"
            // save the positions of each floating 'X'
            for (i, bit) in mask.chars().rev().enumerate().filter(|(_, c)| ['1', 'X'].contains(c)) {
                match bit {
                    '1' => address |= 1 << i,
                    _ => floating_pos.push(i),
                };
            }

            // there are 2^n permutations to set in memory
            // use the bit mask of the values in 0..2^n to set
            // the corresponding positions held in floating_pos
            // Example:
            //   if there are 2 floating 'X's there are 4 combinations represented
            //   by the values 0, 1, 2, 3 or bits 00, 01, 10, 11
            //   00 would set "address" bits at positions floating_pos[0] to 0, and floating_pos[1] to 0
            //   01 would set "address" bits at positions floating_pos[0] to 0, and floating_pos[1] to 1
            //   10 would set "address" bits at positions floating_pos[0] to 1, and floating_pos[1] to 0
            //   11 would set "address" bits at positions floating_pos[0] to 1, and floating_pos[1] to 1
            let num_x = floating_pos.len();
            for mut permutation in 0..(2_u32.pow(num_x as u32)) {
                for pos in (0..num_x).rev() {
                    match permutation % 2 == 0 {
                        true => address &= !(1 << floating_pos[pos]),
                        false => address |= 1 << floating_pos[pos],
                    };
                    permutation >>= 1;
                }
                memory.insert(address, value);
            }
        }
    }
    memory.values().sum()
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
        assert_eq!(165, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(5902420735773, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example2");
        assert_eq!(208, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(3801988250775, solution2(&data));
    }
}
