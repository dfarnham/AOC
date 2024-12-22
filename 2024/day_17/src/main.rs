use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Write};

fn get_program(data: &[String]) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let mut registers = vec![];
    let mut program = vec![];

    for line in data {
        if line.is_empty() {
            continue;
        }
        let field = &trim_split_on::<String>(line, ':')?[1];
        if line.starts_with("Register") {
            registers.push(field.parse::<usize>()?);
        } else if line.starts_with("Program") {
            program = trim_split_on::<usize>(field, ',')?;
        }
    }
    Ok((registers, program))
}

fn run_program(program: &[usize], registers: &mut [usize]) -> Vec<usize> {
    let mut pc = 0;
    let mut output = vec![];

    while pc < program.len() - 1 {
        let mut jmp = false;
        let (opcode, operand) = (program[pc], program[pc + 1]);
        let combo_operand = match operand {
            n if [0, 1, 2, 3].contains(&n) => n,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => unreachable!(),
        };
        //println!("opcode {opcode}, operand {operand}, combo_operand {combo_operand}");

        match opcode {
            // adv
            0 => registers[0] >>= combo_operand,
            // bxl
            1 => registers[1] ^= operand,
            // bst
            2 => registers[1] = combo_operand & 0b111,
            // jnz
            3 => {
                if registers[0] != 0 {
                    pc = operand;
                    jmp = true;
                }
            }
            // bxc
            4 => registers[1] ^= registers[2],
            // out
            5 => output.push(combo_operand & 0b111),
            // bdv
            6 => registers[1] = registers[0] >> combo_operand,
            // cdv
            7 => registers[2] = registers[0] >> combo_operand,
            _ => unreachable!(),
        }
        if !jmp {
            pc += 2
        }
    }
    output
}

#[allow(dead_code)]
fn _dfs(program: &[usize], d: usize, n: usize) -> usize {
    let mut best = usize::MAX;
    if d == 0 || run_program(program, &mut [n, 0, 0]) == program[program.len() - d..] {
        if d == program.len() {
            return n;
        }
        for i in 0..8 {
            best = best.min(_dfs(program, d + 1, n << 3 | i))
        }
    }
    best
}
#[allow(dead_code)]
fn dfs(program: &[usize]) -> usize {
    _dfs(program, 0, 0)
}

fn priorityq(program: &[usize]) -> usize {
    let mut workq = VecDeque::new();
    workq.push_back((0, 0));
    while let Some((d, n)) = workq.pop_front() {
        if d == 0 || run_program(program, &mut [n, 0, 0]) == program[program.len() - d..] {
            if d == program.len() {
                return n;
            }
            for i in 0..8 {
                workq.push_back((d + 1, n << 3 | i));
            }
        }
    }
    usize::MAX
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<String, Box<dyn Error>> {
    let (mut registers, program) = get_program(puzzle_lines)?;
    if !part2 {
        Ok(run_program(&program, &mut registers)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(","))
    } else {
        //assert_eq!(dfs(&program), priorityq(&program));
        Ok(priorityq(&program).to_string())
    }
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

    let n = solve(&puzzle_lines, false)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = solve(&puzzle_lines, true)?;
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
        assert_eq!(solve(&puzzle_lines, false)?, "4,6,3,5,6,3,5,2,1,0".to_string());
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, "1,2,3,1,3,2,5,3,1".to_string());
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, true)?, "117440".to_string());
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, "105706277661082".to_string());
        Ok(())
    }
}
