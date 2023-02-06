use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, Write};

fn get_cycles(puzzle_lines: &[String]) -> Result<Vec<(usize, i32)>, Box<dyn Error>> {
    let mut clock: usize = 0;
    let mut register = 1;
    let mut cycles = vec![];

    for line in puzzle_lines {
        clock += 1;
        let mut cmd = line.split_whitespace();
        if let (Some(instr), Some(value)) = (cmd.next(), cmd.next()) {
            match instr {
                "addx" => {
                    cycles.push((clock, register));
                    register += value.parse::<i32>()?;
                    clock += 1
                }
                _ => return Err(Box::from(format!("Unknown instr: {instr}"))),
            };
        }
        cycles.push((clock, register));
    }
    Ok(cycles)
}

fn signal_strength(puzzle_lines: &[String]) -> Result<i32, Box<dyn Error>> {
    Ok(get_cycles(puzzle_lines)?
        .windows(2)
        .filter(|cycle| cycle[1].0 == 20 || ((cycle[1].0 as i32) - 20) % 40 == 0)
        .map(|cycle| (cycle[1].0 as i32) * cycle[0].1)
        .sum::<i32>())
}

fn crt(puzzle_lines: &[String]) -> Result<Vec<BTreeSet<usize>>, Box<dyn Error>> {
    let crt_width = 40;
    let mut clock: usize = 0;
    let mut register = 1;
    let mut sprite = 0..=3;
    let mut lit = BTreeSet::new();
    let mut screen = vec![];

    for line in puzzle_lines {
        let t = clock % crt_width;
        if sprite.contains(&(t as i32)) {
            lit.insert(t);
        }
        clock += 1;

        let mut cmd = line.split_whitespace();
        if let (Some(instr), Some(value)) = (cmd.next(), cmd.next()) {
            match instr {
                "addx" => {
                    if clock % 40 == 0 {
                        screen.push(lit.clone());
                        lit.clear();
                    }
                    let t = clock % crt_width;
                    if sprite.contains(&(t as i32)) {
                        lit.insert(t);
                    }
                    clock += 1;

                    register += value.parse::<i32>()?;
                    sprite = register - 1..=register + 1
                }
                _ => return Err(Box::from(format!("Unknown instr: {instr}"))),
            };
        }
        if clock % crt_width == 0 {
            screen.push(lit.clone());
            lit.clear()
        }
    }
    Ok(screen)
}

fn display(screen: &Vec<BTreeSet<usize>>) {
    for set in screen {
        for p in 0..40 {
            print!("{}", if set.contains(&p) { "#" } else { " " })
        }
        println!()
    }
}

fn part1(puzzle_lines: &[String]) -> Result<i32, Box<dyn Error>> {
    signal_strength(puzzle_lines)
}

fn part2(puzzle_lines: &[String]) -> Result<Vec<BTreeSet<usize>>, Box<dyn Error>> {
    crt(puzzle_lines)
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

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    let output = part2(&puzzle_lines)?;
    display(&output);
    writeln!(stdout, "Answer Part 2 = {}", output.len())?;

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
        assert_eq!(part1(&puzzle_lines)?, 13140);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 15220);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        let output = part2(&puzzle_lines)?;
        assert_eq!(output.len(), 6);
        assert_eq!(
            output[0],
            BTreeSet::from([
                0, 1, 4, 5, 8, 9, 12, 13, 16, 17, 20, 21, 24, 25, 28, 29, 32, 33, 36, 37
            ])
        );
        assert_eq!(
            output[1],
            BTreeSet::from([
                0, 1, 2, 6, 7, 8, 12, 13, 14, 18, 19, 20, 24, 25, 26, 30, 31, 32, 36, 37, 38
            ])
        );
        assert_eq!(
            output[2],
            BTreeSet::from([
                0, 1, 2, 3, 8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35
            ])
        );
        assert_eq!(
            output[3],
            BTreeSet::from([
                0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24, 30, 31, 32, 33, 34
            ])
        );
        assert_eq!(
            output[4],
            BTreeSet::from([
                0, 1, 2, 3, 4, 5, 12, 13, 14, 15, 16, 17, 24, 25, 26, 27, 28, 29, 36, 37, 38, 39
            ])
        );
        assert_eq!(
            output[5],
            BTreeSet::from([
                0, 1, 2, 3, 4, 5, 6, 14, 15, 16, 17, 18, 19, 20, 28, 29, 30, 31, 32, 33, 34
            ])
        );
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        let output = part2(&puzzle_lines)?;
        assert_eq!(output.len(), 6);
        assert_eq!(
            output[0],
            BTreeSet::from([
                0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 20, 23, 25, 26, 27, 30, 31,
                32, 33, 36, 37
            ])
        );
        assert_eq!(
            output[1],
            BTreeSet::from([0, 3, 5, 13, 15, 20, 22, 25, 28, 30, 35, 38])
        );
        assert_eq!(
            output[2],
            BTreeSet::from([0, 3, 5, 6, 7, 12, 15, 16, 17, 20, 21, 25, 26, 27, 30, 31, 32, 35, 38])
        );
        assert_eq!(
            output[3],
            BTreeSet::from([0, 1, 2, 5, 11, 15, 20, 22, 25, 28, 30, 35, 36, 37, 38])
        );
        assert_eq!(
            output[4],
            BTreeSet::from([0, 2, 5, 10, 15, 20, 22, 25, 28, 30, 35, 38])
        );
        assert_eq!(
            output[5],
            BTreeSet::from([
                0, 3, 5, 10, 11, 12, 13, 15, 16, 17, 18, 20, 23, 25, 26, 27, 30, 35, 38
            ])
        );
        Ok(())
    }
}
