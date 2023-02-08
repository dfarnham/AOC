use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<usize>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

fn run_program(opcodes: &[usize], noun: usize, verb: usize) -> Result<usize, Box<dyn Error>> {
    let mut opcodes = opcodes.to_owned();

    // before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    // What value is left at position 0 after the program halts?
    // In this program, the value placed in address 1 is called the noun,
    // and the value placed in address 2 is called the verb.

    // no bounds checking
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut i = 0;
    while opcodes[i] != 99 {
        let (a, b, c) = (opcodes[i + 1], opcodes[i + 2], opcodes[i + 3]);
        opcodes[c] = match opcodes[i] {
            1 => opcodes[a] + opcodes[b],
            2 => opcodes[a] * opcodes[b],
            n => panic!("invalid opcode: {n}"),
        };
        i += 4;
    }
    Ok(opcodes[0])
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let opcodes = get_data(puzzle_lines)?;
    run_program(&opcodes, 12, 2)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let opcodes = get_data(puzzle_lines)?;

    for noun in 0..99 {
        for verb in 0..99 {
            if run_program(&opcodes, noun, verb)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("no solution".into())
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
        assert_eq!(part1(&puzzle_lines)?, 8017076);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 3146);
        Ok(())
    }
}
