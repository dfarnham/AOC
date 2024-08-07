use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use pathfinding::matrix::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<i64>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

#[rustfmt::skip]
fn run_program(program: &[i64], input: &[u32], grid: &mut Vec<char>) -> Result<usize, Box<dyn Error>> {
    let mut opcodes = BTreeMap::<i64, i64>::new();
    for (inst_ptr, code) in program.iter().enumerate() {
        opcodes.insert(inst_ptr as i64, *code);
    }

    let mut relative_base = 0;
    let mut inst_ptr = 0;
    let mut input_index = 0;
    let mut result = 0;

    while opcodes[&inst_ptr] != 99 {
        // ***********************************************
        // there is no bounds checking on indexing opcodes
        // ***********************************************

        // skip the 2 digit instruction and gather all modes (modes are listed R->L)
        let mut modes = vec![];
        let mut n = 1000 + opcodes[&inst_ptr] / 100;
        for _ in 0..3 {
            modes.push(n % 10);
            n /= 10;
        }

        // the numeric opcode
        let opcode = opcodes[&inst_ptr] % 100;

        // Opcode 1,2 either add or multiply numbers read from
        // two positions and stores the result in a third position.
        //
        // Opcode 3 takes a single integer as input and saves
        // it to the position given by its only parameter.
        //
        // Opcode 4 outputs the value of its only parameter.
        //
        // Opcode 5 is jump-if-true, if the first parameter is non-zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        //
        // Opcode 6 is jump-if-false, if the first parameter is zero, it sets
        // the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        //
        // Opcode 7 is less than: if the first parameter is less than the second
        // parameter, it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        //
        // Opcode 8 is equals: if the first parameter is equal to the second
        // parameter, it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        //
        // Opcode 9 adjusts the relative base by the value of its only parameter.
        // The relative base increases (or decreases, if the value is negative)
        // by the value of the parameter.
        //
        // Parameter modes:
        //   0 - position mode - the parameter to be interpreted as a position
        //
        //   1 - immediate mode - the parameter is interpreted as a value
        //
        //   2 - relative mode - use offset from relative base,
        //                       the parameter is interpreted as a position.
        //
        // Memory: Memory beyond the initial program starts with the
        //         value 0 and can be read or written like any other memory.
        //         (It is invalid to try to access memory at a negative address, though.)

        // parameters
        let param_1 = *opcodes.entry(inst_ptr + 1).or_default();
        let param_2 = *opcodes.entry(inst_ptr + 2).or_default();
        let param_3 = *opcodes.entry(inst_ptr + 3).or_default();

        // first param value is needed by most opcodes
        let a = match modes[0] {
            0 => *opcodes.entry(param_1).or_default(),
            1 => param_1,
            2 => *opcodes.entry(param_1 + relative_base).or_default(),
            _ => panic!("opcode = {opcode}, modes = {modes:?}"),
        };

        match opcode {
            1 | 2 | 5 | 6 | 7 | 8 => {
                // second param value
                let b = match modes[1] {
                    0 => *opcodes.entry(param_2).or_default(),
                    1 => param_2,
                    2 => *opcodes.entry(param_2 + relative_base).or_default(),
                    _ => panic!("opcode = {opcode}, modes = {modes:?}"),
                };

                if opcode == 5 {
                    // instruction pointer set to second param or increases by 3
                    inst_ptr = if a != 0 { b } else { inst_ptr + 3 }
                } else if opcode == 6 {
                    // instruction pointer set to second param or increases by 3
                    inst_ptr = if a == 0 { b } else { inst_ptr + 3 }
                } else {
                    // third param: destination index
                    let index = match modes[2] {
                        0 | 1 => param_3,
                        2 => param_3 + relative_base,
                        _ => panic!("opcode = {opcode}, modes = {modes:?}"),
                    };

                    opcodes.insert(
                        index,
                        match opcode {
                            1 => a + b,
                            2 => a * b,
                            7 => if a < b { 1 } else { 0 },
                            8 => if a == b { 1 } else { 0 },
                            _ => unreachable!(),
                        },
                    );

                    // instruction pointer increases by 4
                    inst_ptr += 4;
                }
            }
            9 => {
                // first param value
                relative_base += a;

                // instruction pointer increases by 2
                inst_ptr += 2;
            }
            3 | 4 => {
                if opcode == 3 {
                    // first param: destination index
                    let index = match modes[0] {
                        0 | 1 => param_1,
                        2 => param_1 + relative_base,
                        _ => panic!("opcode = {opcode}, modes = {modes:?}"),
                    };
                    print!("{}", char::from_u32(input[input_index]).unwrap());
                    opcodes.insert(index, input[input_index] as i64);
                    input_index += 1;
                } else if input.is_empty() {
                    grid.push(char::from_u32(a as u32).unwrap());
                } else {
                        if a < 256 {
                            print!("{}", char::from_u32(a as u32).unwrap());
                        }
                        result = a;
                }

                // instruction pointer increases by 2
                inst_ptr += 2;
            }
            _ => return Err(format!("invalid opcode: {opcode}").into()),
        }
    }

    Ok(result as usize)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    let mut grid_data = vec![];
    run_program(&program, &[], &mut grid_data)?;
    grid_data.pop(); // remove trailing newline

    // build a Matrix
    let mut rows = vec![];
    let mut row = vec![];
    for c in grid_data {
        if c == '\n' {
            rows.push(row.clone());
            row.clear();
        } else {
            row.push(c);
        }
    }
    let grid = Matrix::from_rows(&rows)?;

    // find intersections and display the grid
    let mut sum = 0;
    for i in 0..grid.rows {
        for j in 0..grid.columns {
            print!("{}", grid[(i, j)]);
            if *grid[(i, j)] == '#'
                && i > 0
                && i < grid.rows - 1
                && j > 0
                && j < grid.columns - 1
                && *grid[(i - 1, j)] == '#'
                && *grid[(i + 1, j)] == '#'
                && *grid[(i, j - 1)] == '#'
                && *grid[(i, j + 1)] == '#'
            {
                sum += i * j;
            }
        }
        println!();
    }

    Ok(sum)
}

// Solved by hand
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut program = get_data(puzzle_lines)?;
    let mut grid_data = vec![];
    program[0] = 2;

    // L8 R10 L8 R8 L12 R8 R8 L8 R10 L8 R8 L8 R6 R6 R10 L8 L8 R6 R6 R10 L8 L8 R10 L8 R8 L12 R8 R8 L8 R6 R6 R10 L8 L12 R8 R8 L12 R8 R8
    // [     A    ] [   B   ] [     A    ] [     C       ] [     C       ] [     A    ] [   B   ] [     C       ] [   B   ] [   B   ]
    //
    // A = L8 R10 L8 R8
    // B = L12 R8 R8
    // C = L8 R6 R6 R10 L8
    let input = "A,B,A,C,C,A,B,C,B,B\nL,8,R,10,L,8,R,8\nL,12,R,8,R,8\nL,8,R,6,R,6,R,10,L,8\nN\n";
    let input: Vec<u32> = input.chars().map(|c| c as u32).collect();
    let result = run_program(&program, &input, &mut grid_data)?;
    Ok(result as usize)
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
        assert_eq!(part1(&puzzle_lines)?, 6680);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1103905);
        Ok(())
    }
}
