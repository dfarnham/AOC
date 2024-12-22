use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Write};

fn get_program(data: &[String]) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let mut register = vec![];
    let mut program = vec![];

    for line in data {
        if line.is_empty() {
            continue;
        }
        let field = &trim_split_on::<String>(line, ':')?[1];
        if line.starts_with("Register") {
            register.push(field.parse::<usize>()?);
        } else if line.starts_with("Program") {
            program = trim_split_on::<usize>(field, ',')?;
        }
    }
    Ok((register, program))
}

fn run_program(program: &[usize], register: &mut [usize]) -> Vec<usize> {
    let mut pc = 0;
    let mut output = vec![];

    while pc < program.len() {
        let mut jump = false;
        let opcode = program[pc];
        let operand = program[pc + 1];
        let combo_operand = match operand {
            n if [0, 1, 2, 3].contains(&n) => n,
            4 => register[0],
            5 => register[1],
            6 => register[2],
            _ => unreachable!(),
        };
        //println!("opcode {opcode}, operand {operand}, combo_operand {combo_operand}");

        match opcode {
            // adv
            0 => register[0] /= 2_usize.pow(combo_operand as u32),
            // bxl
            1 => register[1] ^= operand,
            // bst
            2 => register[1] = combo_operand % 8,
            // jnz
            3 => {
                if register[0] != 0 {
                    pc = operand;
                    jump = true;
                }
            }
            // bxc
            4 => register[1] ^= register[2],
            // out
            5 => output.push(combo_operand % 8),
            // bdv
            6 => register[1] = register[0] / 2_usize.pow(combo_operand as u32),
            // cdv
            7 => register[2] = register[0] / 2_usize.pow(combo_operand as u32),
            _ => unreachable!(),
        }
        if !jump {
            pc += 2
        }
    }
    output
}

fn dfs(program: &[usize], d: usize, v: usize) -> usize {
    let mut best = usize::MAX;
    let mut register = vec![v, 0, 0];
    if d == 0 || run_program(program, &mut register) == program[program.len() - d..].to_vec() {
        if d == program.len() {
            return v;
        } else {
            for i in 0..8 {
                best = best.min(dfs(program, d + 1, v << 3 | i))
            }
        }
    }
    best
}

fn priorityq(program: &[usize]) -> usize {
    let mut best = usize::MAX;
    let mut workq = VecDeque::new();
    workq.push_back((0, 0));
    while let Some((d, v)) = workq.pop_front() {
        let mut register = vec![v, 0, 0];
        if d == 0 || run_program(program, &mut register) == program[program.len() - d..].to_vec() {
            if d == program.len() {
                best = v;
                break;
            }
            for i in 0..8 {
                workq.push_back((d + 1, v << 3 | i));
            }
        }
    }
    best
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<String, Box<dyn Error>> {
    let (mut register, program) = get_program(puzzle_lines)?;
    if !part2 {
        Ok(run_program(&program, &mut register)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .to_string())
    } else {
        let best = dfs(&program, 0, 0);
        assert_eq!(best, priorityq(&program));
        Ok(best.to_string())
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
