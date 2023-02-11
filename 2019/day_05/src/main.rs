use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<i64>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

#[rustfmt::skip]
fn run_program(program: &[i64], input: i64) -> Result<i64, Box<dyn Error>> {
    let mut opcodes = program.to_owned();

    let mut result = 0;
    let mut i = 0;
    while i < opcodes.len() && opcodes[i] != 99 {
        // ***********************************************
        // there is no bounds checking on indexing opcodes
        // ***********************************************

        // skip the 2 digit instruction and gather all modes (modes are listed R->L)
        let mut modes = vec![];
        let mut n = 10000 + opcodes[i] / 100;
        while n > 0 {
            modes.push(n % 10);
            n /= 10;
        }

        // the numeric opcode
        let opcode = opcodes[i] % 100;

        match opcode {
            1 | 2 | 5 | 6 | 7 | 8 => {
                // first param
                let a = match modes[0] == 1 {
                    true => opcodes[i + 1],
                    false => opcodes[opcodes[i + 1] as usize],
                };

                // second param
                let b = match modes[1] == 1 {
                    true => opcodes[i + 2],
                    false => opcodes[opcodes[i + 2] as usize],
                };

                if opcode == 5 {
                    i = if a != 0 { b as usize } else { i + 3 }
                } else if opcode == 6 {
                    i = if a == 0 { b as usize } else { i + 3 }
                } else {
                    // third param: destination index
                    let c = opcodes[i + 3] as usize;

                    opcodes[c] = match opcode {
                        1 => a + b,
                        2 => a * b,
                        7 => if a < b { 1 } else { 0 }
                        8 => if a == b { 1 } else { 0 }
                        _ => panic!("wtf"),
                    };

                    // instruction pointer moves by 4
                    i += 4;
                }
            }
            3 => {
                // set supplied input at the next param (index)
                let a = opcodes[i + 1] as usize;
                opcodes[a] = input;
                i += 2;
            }
            4 => {
                // output value at the next param (index)
                result = opcodes[opcodes[i + 1] as usize];
                println!("{result}");
                i += 2;
            }
            _ => return Err("invalid opcode".into()),
        }
    }

    Ok(result)
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    run_program(&program, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    run_program(&program, 5)
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
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 7692125);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 14340395);
        Ok(())
    }
}
