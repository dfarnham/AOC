use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

/// PartNumber - field data about a part-number in the grid
/// postion: (i,j)
/// numeric value: n
///
/// methods
///   len(): number of digits
///   adjacents(): surrounding coordinates within the grid that touch the number
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct PartNumber {
    i: usize, // i coordinate
    j: usize, // j coordinate
    n: usize, // value
}
impl PartNumber {
    fn new(i: usize, j: usize, n: usize) -> Self {
        Self { i, j, n }
    }
    fn len(&self) -> usize {
        self.n.to_string().len()
    }
    fn adjacents(&self) -> HashSet<(usize, usize)> {
        let mut coords = HashSet::new();
        for j in self.j..self.j + self.len() {
            for (x, y) in [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let a = x as i64 + self.i as i64;
                let b = y as i64 + j as i64;
                if a >= 0 && b >= 0 {
                    coords.insert((a as usize, b as usize));
                }
            }
        }
        coords
    }
}

fn get_grid(data: &[String]) -> Array2<char> {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), '.');

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }
    grid
}

fn part_numbers_in_grid(grid: &Array2<char>) -> Vec<PartNumber> {
    let numeric = |digits: &[u32]| digits.iter().fold(0, |acc, d| acc * 10 + *d as usize);

    let (nrow, ncol) = grid.dim();
    let mut part_numbers = vec![];

    for i in 0..nrow {
        let mut digits = vec![];
        for j in 0..ncol {
            let c = grid[[i, j]];
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).expect("char digit"));
            } else if !digits.is_empty() {
                part_numbers.push(PartNumber::new(i, j - digits.len(), numeric(&digits)));
                digits.clear();
            }
        }
        if !digits.is_empty() {
            part_numbers.push(PartNumber::new(i, ncol - 1 - digits.len(), numeric(&digits)));
        }
    }
    part_numbers
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let is_symbol = |c: char| !(c.is_ascii_digit() || c == '.');

    let grid = get_grid(puzzle_lines);
    let symbols: HashSet<(usize, usize)> = grid
        .indexed_iter()
        .filter(|tup| is_symbol(*tup.1))
        .map(|tup| tup.0)
        .collect();

    Ok(part_numbers_in_grid(&grid)
        .iter()
        .filter(|pn| pn.adjacents().iter().any(|adj| symbols.contains(adj)))
        .map(|pn| pn.n)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let is_gear = |c: char| c == '*';

    let grid = get_grid(puzzle_lines);
    let mut symbols: HashMap<(usize, usize), Vec<usize>> = grid
        .indexed_iter()
        .filter(|tup| is_gear(*tup.1))
        .map(|tup| (tup.0, vec![]))
        .collect();

    for pn in part_numbers_in_grid(&grid) {
        for adj in pn.adjacents() {
            if let Some(touching) = symbols.get_mut(&adj) {
                touching.push(pn.n);
            }
        }
    }

    Ok(symbols
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<usize>())
        .sum())
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

    let n = part1(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = part2(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 2 = {n}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file = std::path::PathBuf::from(filename);
        Ok(read_trimmed_data_lines(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part1(&puzzle_lines)?, 4361);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 527364);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 467835);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 79026871);
        Ok(())
    }
}
