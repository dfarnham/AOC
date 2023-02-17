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
    let mut inst_ptr = 0;
    while opcodes[inst_ptr] != 99 {
        // ***********************************************
        // there is no bounds checking on indexing opcodes
        // ***********************************************

        // skip the 2 digit instruction and gather all modes (modes are listed R->L)
        let mut modes = vec![];
        let mut n = 10000 + opcodes[inst_ptr] / 100;
        while n > 0 {
            modes.push(n % 10);
            n /= 10;
        }

        // the numeric opcode
        let opcode = opcodes[inst_ptr] % 100;

        match opcode {
            1 | 2 | 5 | 6 | 7 | 8 => {
                // first param value
                let a = match modes[0] {
                    0 => opcodes[opcodes[inst_ptr + 1] as usize],
                    1 => opcodes[inst_ptr + 1],
                    _ => panic!("opcode = {opcode}, modes = {modes:?}"),
                };

                // second param value
                let b = match modes[1] {
                    0 => opcodes[opcodes[inst_ptr + 2] as usize],
                    1 => opcodes[inst_ptr + 2],
                    _ => panic!("opcode = {opcode}, modes = {modes:?}"),
                };

                if opcode == 5 {
                    // instruction pointer set to second param or increases by 3
                    inst_ptr = if a != 0 { b as usize } else { inst_ptr + 3 }
                } else if opcode == 6 {
                    // instruction pointer set to second param or increases by 3
                    inst_ptr = if a == 0 { b as usize } else { inst_ptr + 3 }
                } else {
                    // third param: destination index
                    let index = opcodes[inst_ptr + 3] as usize;

                    opcodes[index] = match opcode {
                        1 => a + b,
                        2 => a * b,
                        7 => if a < b { 1 } else { 0 },
                        8 => if a == b { 1 } else { 0 },
                        _ => panic!("wtf"),
                    };

                    // instruction pointer increases by 4
                    inst_ptr += 4;
                }
            }
            3 | 4 => {
                // first param value
                let index = opcodes[inst_ptr + 1] as usize;

                if opcode == 3 {
                    // set supplied input at param (index)
                    opcodes[index] = input;
                } else {
                    // result, output value at first param index
                    result = match modes[0] {
                        0 => opcodes[index],
                        1 => opcodes[inst_ptr + 1],
                        _ => panic!("opcode = {opcode}, modes = {modes:?}")
                    };
                    println!("{result}");
                }

                // instruction pointer increases by 2
                inst_ptr += 2;
            }
            _ => return Err(format!("invalid opcode: {opcode}").into()),
        }
    }

    // return last stored result
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
