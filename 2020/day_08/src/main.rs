use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

fn get_program(data: &[String]) -> Vec<Instruction> {
    let mut prog = vec![];

    for line in data {
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        let op = match tokens[0] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!("oops"),
        };
        let arg = tokens[1].parse::<i32>().expect("failed parse");
        prog.push(Instruction { op, arg });
    }
    prog
}

fn solution1(prog: &[Instruction]) -> (i32, usize) {
    let mut executed = HashSet::new();
    let mut inst_ptr = 0;
    let mut accumulator = 0;

    while !executed.contains(&inst_ptr) {
        executed.insert(inst_ptr);
        match prog[inst_ptr].op {
            Operation::Acc => {
                accumulator += prog[inst_ptr].arg;
                inst_ptr += 1;
            }
            Operation::Jmp => {
                inst_ptr = (prog[inst_ptr].arg + inst_ptr as i32) as usize;
            }
            Operation::Nop => {
                inst_ptr += 1;
            }
        }

        if inst_ptr == prog.len() {
            break; // prog end
        }
    }
    (accumulator, inst_ptr)
}

fn solution2(prog: &[Instruction]) -> i32 {
    for (i, stmt) in prog.iter().enumerate() {
        if stmt.op == Operation::Jmp || stmt.op == Operation::Nop {
            let mut d = prog.to_vec();
            d[i].op = match d[i].op {
                Operation::Nop => Operation::Jmp,
                _ => Operation::Nop,
            };
            let (accumulator, inst_ptr) = solution1(&d);
            if inst_ptr == prog.len() {
                return accumulator;
            }
        }
    }
    panic!("no solution")
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

    let prog = get_program(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&prog))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&prog))?;

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
        let prog = get_program(&data);
        assert_eq!(5, solution1(&prog).0);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let prog = get_program(&data);
        assert_eq!(2025, solution1(&prog).0);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let prog = get_program(&data);
        assert_eq!(8, solution2(&prog));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let prog = get_program(&data);
        assert_eq!(2001, solution2(&prog));
    }
}
