use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn get_data(puzzle_lines: &[String]) -> Result<Vec<i64>, Box<dyn Error>> {
    trim_split_on(&puzzle_lines[0], ',')
}

#[rustfmt::skip]
fn run_program(program: &[i64], input_phases: &Vec<usize>, feedback: bool) -> Result<i64, Box<dyn Error>> {
    let mut amplifiers = VecDeque::new();
    let mut phase_init = HashSet::new();

    // build a work queue of amplifiers
    //
    // each amplifier has:
    //   * a phase (needed once to initialize),
    //   * instruction pointer into modified bytecodes
    for phase in input_phases {
        amplifiers.push_back((*phase as i64, 0, program.to_owned()));
    }

    // the signal being output from an amplifier and used as input for the next amplifier
    let mut shared_signal = 0;

    // process the work queue
    while let Some((phase, mut inst_ptr, mut opcodes)) = amplifiers.pop_front() {
        // process opcode instructions until told to stop with 99
        while inst_ptr < opcodes.len() && opcodes[inst_ptr] != 99 {
            // ***********************************************
            // there is no bounds checking on indexing opcodes
            // ***********************************************

            // skip the 2 digit instruction and gather all modes (modes are listed R->L)
            let mut modes = vec![];
            let mut n = 1000 + opcodes[inst_ptr] / 100;
            for _ in 0..3 {
                modes.push(n % 10);
                n /= 10;
            }

            // the numeric opcode
            let opcode = opcodes[inst_ptr] % 100;

            // first param value is needed by most opcodes
            let a = match modes[0] == 1 {
                true => opcodes[inst_ptr + 1],
                false => opcodes[opcodes[inst_ptr + 1] as usize],
            };

            match opcode {
                1 | 2 | 5 | 6 | 7 | 8 => {
                    // second param
                    let b = match modes[1] == 1 {
                        true => opcodes[inst_ptr + 2],
                        false => opcodes[opcodes[inst_ptr + 2] as usize],
                    };

                    if opcode == 5 {
                        // instruction pointer set to second param or increases by 3
                        inst_ptr = if a != 0 { b as usize } else { inst_ptr + 3 }
                    } else if opcode == 6 {
                        // instruction pointer set to second param or increases by 3
                        inst_ptr = if a == 0 { b as usize } else { inst_ptr + 3 }
                    } else {
                        // third param: destination index
                        let c = opcodes[inst_ptr + 3] as usize;

                        opcodes[c] = match opcode {
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
                    // first param: destination index
                    let index = opcodes[inst_ptr + 1] as usize;

                    // instruction pointer increases by 2
                    inst_ptr += 2;

                    if opcode == 3 {
                        // input the current signal or initialize
                        opcodes[index] = match phase_init.contains(&phase) {
                            true => shared_signal,
                            false => {
                                phase_init.insert(phase);
                                phase
                            }
                        };
                    } else {
                        // output value at param (index)
                        shared_signal = a;

                        // the amplifiers are just a work queue, they suspend
                        // their work state and go to the end of the queue,
                        //
                        // but... they're burdened with the task of forcing a work
                        // stopage by advancing the instruction pointer to the end
                        // [ or a continue to the outer while() if i labeled it ]
                        if feedback {
                            amplifiers.push_back((phase, inst_ptr, opcodes.to_owned()));
                            inst_ptr = opcodes.len();
                            continue;
                        }
                    }
                }
                _ => return Err("invalid opcode".into()),
            }
        }
    }

    Ok(shared_signal)
}

fn solution(program: &[i64], phases: &[usize], feedback: bool) -> Result<i64, Box<dyn Error>> {
    // max signal over permutations
    Ok(phases
        .iter()
        .permutations(phases.len())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|perm| perm.into_iter().copied().collect())
        .map(|phases| run_program(program, &phases, feedback).expect("bad gram"))
        .max()
        .expect("no max"))
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    let phases = [0, 1, 2, 3, 4];
    let feedback = false;
    solution(&program, &phases, feedback)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let program = get_data(puzzle_lines)?;
    let phases = [5, 6, 7, 8, 9];
    let feedback = true;
    solution(&program, &phases, feedback)
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
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 43210);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 116680);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 98765);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 89603079);
        Ok(())
    }
}
