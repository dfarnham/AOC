use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<usize>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

fn run_program(opcodes: &[usize]) -> Result<usize, Box<dyn Error>> {
    let mut opcodes = opcodes.to_owned();

    let mut i = 0;
    while opcodes[i] != 99 {
        let (a, b, c) = (opcodes[i + 1], opcodes[i + 2], opcodes[i + 3]);

        // Opcode 1,2 either add or multiply numbers read from
        // two positions and stores the result in a third position.
        opcodes[c] = match opcodes[i] {
            1 => opcodes[a] + opcodes[b],
            2 => opcodes[a] * opcodes[b],
            n => panic!("invalid opcode: {n}"),
        };

        // move the instruction pointer forward by 4 positions
        i += 4;
    }

    // What value is left at position 0 after the program halts?
    Ok(opcodes[0])
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut opcodes = get_data(puzzle_lines)?;

    // before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    opcodes[1] = 12;
    opcodes[2] = 2;

    // What value is left at position 0 after the program halts?
    run_program(&opcodes)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut opcodes = get_data(puzzle_lines)?;

    // In this program, the value placed in address 1 is called the noun,
    // and the value placed in address 2 is called the verb.
    // Each of the two input values will be between 0 and 99, inclusive.
    for noun in 0..99 {
        for verb in 0..99 {
            // In this program, the value placed in address 1 is called the noun,
            // and the value placed in address 2 is called the verb.
            //
            // Find the input noun and verb that cause the
            // program to produce the output 19690720
            opcodes[1] = noun;
            opcodes[2] = verb;
            if run_program(&opcodes)? == 19690720 {
                // What is 100 * noun + verb?
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
