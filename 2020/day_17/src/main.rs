use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Cubespace {
    active: HashSet<(i64, i64, i64, i64)>,
    sz: usize,
}
impl Cubespace {
    fn new(sz: usize) -> Self {
        Self {
            active: HashSet::new(),
            sz,
        }
    }
    fn activate(&mut self, x: i64, y: i64, z: i64, w: i64) {
        self.active.insert((x, y, z, w));
    }
    fn deactivate(&mut self, x: i64, y: i64, z: i64, w: i64) {
        self.active.remove(&(x, y, z, w));
    }
    fn is_active(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
        self.active.contains(&(x, y, z, w))
    }
    fn nactive(&self) -> usize {
        self.active.len()
    }
}

fn init(data: &[String]) -> Cubespace {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().map(|c| c == '#').collect::<Vec<_>>();

    let mut cubespace = Cubespace::new(data[0].len());
    for (row, line) in data.iter().enumerate() {
        for (col, _) in get_row(line).into_iter().enumerate().filter(|(_, state)| *state) {
            cubespace.activate(row as i64, col as i64, 0, 0);
        }
    }
    cubespace
}

fn adjacent_active(x: i64, y: i64, z: i64, w: i64, cubespace: &Cubespace, dim: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                count += (match dim {
                    3 => 0..=0,
                    _ => -1..=1,
                })
                .filter(|l| cubespace.is_active(x + i, y + j, z + k, w + l))
                .count();
            }
        }
    }
    match cubespace.is_active(x, y, z, w) && count > 0 {
        true => count - 1,
        false => count,
    }
}

fn solution(cubespace: &mut Cubespace, dim: usize) -> usize {
    assert!(dim == 3 || dim == 4, "unsupported dimension = {dim}");
    let mut updated_cubespace = cubespace.clone();
    let sz = cubespace.sz as i64;
    for cycle in 1..=6 {
        for i in 0 - cycle..=sz + cycle {
            for j in 0 - cycle..=sz + cycle {
                for k in 0 - cycle..=sz + cycle {
                    let wrange = match dim {
                        3 => 0..=0,
                        _ => 0 - cycle..=sz + cycle,
                    };
                    for l in wrange {
                        let adjacent = adjacent_active(i, j, k, l, cubespace, dim);
                        match cubespace.is_active(i, j, k, l) {
                            true => {
                                if !(adjacent == 2 || adjacent == 3) {
                                    updated_cubespace.deactivate(i, j, k, l);
                                }
                            }
                            false => {
                                if adjacent == 3 {
                                    updated_cubespace.activate(i, j, k, l);
                                }
                            }
                        }
                    }
                }
            }
        }
        *cubespace = updated_cubespace.clone();
    }
    cubespace.nactive()
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

    writeln!(stdout, "Answer Part 1 = {:?}", solution(&mut init(&puzzle_lines), 3))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution(&mut init(&puzzle_lines), 4))?;

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
        assert_eq!(112, solution(&mut init(&data), 3));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(338, solution(&mut init(&data), 3));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(848, solution(&mut init(&data), 4));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(2440, solution(&mut init(&data), 4));
    }
}
