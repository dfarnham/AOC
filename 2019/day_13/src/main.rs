use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<i64>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

#[rustfmt::skip]
fn run_program(program: &[i64], screen: &mut Vec<(i64, i64, i64)>, input: i64) -> Result<i64, Box<dyn Error>> {
    let mut opcodes = BTreeMap::<i64, i64>::new();
    for (inst_ptr, code) in program.iter().enumerate() {
        opcodes.insert(inst_ptr as i64, *code);
    }

    let mut input = input;
    let mut relative_base = 0;
    let mut result = 0;
    let mut inst_ptr = 0;
    let mut outputs = vec![];
    let mut paddle_x = 0;
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
                            7 => { if a < b { 1 } else { 0 } }
                            8 => { if a == b { 1 } else { 0 } }
                            _ => panic!("wtf"),
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
                    opcodes.insert(index, input);
                } else {
                    // result, output value at first param
                    result = a;

                    // ======================================================
                    // inline hacking the joystick control on every 3 outputs
                    // ======================================================
                    outputs.push(result);
                    if outputs.len() == 3 {
                        let (x, y, id) = (outputs[0], outputs[1], outputs[2]);

                        // when three output instructions specify X=-1, Y=0, the third output instruction is
                        // not a tile; the value instead specifies the new score to show in the segment display
                        if x == -1 && y == 0 {
                            result = id; // current score
                        } else {
                            if id == 3 { // paddle
                                paddle_x = x;
                            } else if id == 4 { // ball
                                // setting input adjusts joystick
                                input = match paddle_x.cmp(&x) {
                                    Ordering::Greater => -1, // tilt left
                                    Ordering::Less => 1,     // tilt right
                                    Ordering::Equal => 0,    // neutral
                                };
                            }

                            // current position
                            screen.push((x, y, id));
                        }

                        // clear the processed outputs
                        outputs.clear();
                    }
                    // ======================================================
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

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    let mut screen = Vec::<(i64, i64, i64)>::new();
    let _result = run_program(&program, &mut screen, 0);
    Ok(screen.iter().filter(|(_x, _y, id)| *id == 2).count())
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let mut program = get_data(puzzle_lines)?;
    // Memory address 0 represents the number of quarters that
    // have been inserted; set it to 2 to play for free.
    program[0] = 2;
    run_program(&program, &mut Vec::<(i64, i64, i64)>::new(), 0)
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
        assert_eq!(part1(&puzzle_lines)?, 265);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 13331);
        Ok(())
    }
}
