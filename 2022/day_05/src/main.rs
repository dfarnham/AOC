use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};

fn build_stacks(puzzle_lines: &[String]) -> BTreeMap<usize, Vec<char>> {
    let mut stacks = BTreeMap::new();
    for line in puzzle_lines.iter() {
        if line.starts_with(" 1") {
            break;
        }
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                let v: &mut Vec<char> = stacks.entry(i).or_default();
                v.insert(0, c)
            }
        }
    }
    stacks
}

fn move_crates(
    puzzle_lines: &[String],
    stacks: &BTreeMap<usize, Vec<char>>,
    challenge: usize, // 1 or 2
) -> Result<String, Box<dyn Error>> {
    // ** input-example **
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    //
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2

    //
    // stacks: {0: ['Z', 'N'], 1: ['M', 'C', 'D'], 2: ['P']}
    //
    // compute the maximum vector length and skip(n + 2) lines
    // from the input puzzle_lines to start reading the move instructions
    let n = stacks.values().map(|v| v.len()).max().ok_or("max error")?;
    let mut stacks = stacks.clone();

    for line in puzzle_lines.iter().skip(n + 2) {
        let instructions = line.split_whitespace().collect::<Vec<_>>();
        let count = instructions[1].parse::<usize>()?;
        let source = instructions[3].parse::<usize>()? - 1;
        let destination = instructions[5].parse::<usize>()? - 1;

        let mut tmp = vec![];
        for _ in 0..count {
            if let Some(a) = stacks.get_mut(&source) {
                if challenge == 1 {
                    tmp.push(a.pop().ok_or("pop() error")?)
                } else {
                    tmp.insert(0, a.pop().ok_or("pop() error")?)
                }
            }
        }
        if let Some(b) = stacks.get_mut(&destination) {
            b.extend(tmp)
        }
    }

    Ok(stacks
        .values()
        .map(|v| v.last().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(""))
}

fn part1(puzzle_lines: &[String], stacks: &BTreeMap<usize, Vec<char>>) -> Result<String, Box<dyn Error>> {
    move_crates(puzzle_lines, stacks, 1)
}

fn part2(puzzle_lines: &[String], stacks: &BTreeMap<usize, Vec<char>>) -> Result<String, Box<dyn Error>> {
    move_crates(puzzle_lines, stacks, 2)
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

    let stacks = build_stacks(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines, &stacks)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines, &stacks)?)?;

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
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(part1(&puzzle_lines, &stacks)?, "CMZ");
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(part1(&puzzle_lines, &stacks)?, "CVCWCRTVQ");
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(part2(&puzzle_lines, &stacks)?, "MCD");
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(part2(&puzzle_lines, &stacks)?, "CNSCZWLVT");
        Ok(())
    }
}
