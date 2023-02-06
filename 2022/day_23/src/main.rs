use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> HashSet<(i64, i64)> {
    let mut positions = HashSet::new();
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((i as i64, j as i64));
            }
        }
    }
    positions
}

#[rustfmt::skip]
fn solve(start_positions: &HashSet<(i64, i64)>, part: usize) -> Result<usize, Box<dyn Error>> {
    let mut positions = HashMap::new();
    for (e, pos) in start_positions.iter().enumerate() {
        positions.insert(e, *pos);
    }

    //  2 3 4
    //  1 . 5
    //  8 7 6
    //             1         2         3        4       5       6       7       8
    let offs = [ (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), ];
    let (w, nw, n, ne, e, se, s, sw) = (0, 1, 2, 3, 4, 5, 6, 7);
    let directions = [[n, ne, nw], [s, se, sw], [w, nw, sw], [e, ne, se]];

    let mut round = 0;
    loop {
        let mut proposals = HashMap::new();
        let mut destinations = HashMap::new();
        let pset = positions.values().copied().collect::<HashSet<(_, _)>>();

        // first half
        for (e, pos) in &positions {
            let (i, j) = pos;
            if offs.iter().filter(|(r, c)| pset.contains(&(i + r, j + c))).count() > 0 {
                for dir in 0..4 {
                    let idx = (dir + (round % 4)) % 4;
                    let dirs = directions[idx];
                    if dirs.iter().filter(|&d| pset.contains(&(i + offs[*d].0, j + offs[*d].1))).count() == 0 {
                        // dirs[0] is n, s, w, or e
                        let dest = (i + offs[dirs[0]].0, j + offs[dirs[0]].1);
                        *destinations.entry(dest).or_insert(0) += 1;
                        proposals.insert(dest, *e);
                        break;
                    }
                }
            }
        }

        round += 1;

        // second half
        let mut moved = false;
        for (dest, _) in destinations.iter().filter(|(_, count)| *count == &1) {
            let elf = proposals[dest];
            positions.insert(elf, *dest);
            moved = true;
        }
        if part == 1 && round == 10 {
            let minr = positions.values().map(|(r, _)| r).min().unwrap();
            let maxr = positions.values().map(|(r, _)| r).max().unwrap();
            let minc = positions.values().map(|(_, c)| c).min().unwrap();
            let maxc = positions.values().map(|(_, c)| c).max().unwrap();
            let area = (maxr - minr + 1) * (maxc - minc + 1);
            return Ok(area as usize - positions.len())
        } else if part == 2 && !moved {
            return Ok(round);
        }
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let start_positions = get_data(puzzle_lines);
    solve(&start_positions, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let start_positions = get_data(puzzle_lines);
    solve(&start_positions, 2)
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
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

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
        assert_eq!(part1(&puzzle_lines)?, 110);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 4158);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 20);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1014);
        Ok(())
    }
}
