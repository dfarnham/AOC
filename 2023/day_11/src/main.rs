use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use itertools::Itertools;
use ndarray::{Array, Array2, ArrayView};
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Array2<char>, Box<dyn Error>> {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), '.');

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line)))?
    }
    Ok(grid)
}

fn expanded_galaxies(puzzle_lines: &[String], factor: usize) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let empty_row_indices: Vec<_> = (0..grid.nrows())
        .filter(|i| grid.row(*i).iter().all(|elem| *elem == '.'))
        .collect();
    let empty_column_indices: Vec<_> = (0..grid.ncols())
        .filter(|i| grid.column(*i).iter().all(|elem| *elem == '.'))
        .collect();

    let mut galaxies = vec![];
    let mut row_expansion = 0;
    for i in 0..grid.nrows() {
        if empty_row_indices.contains(&i) {
            row_expansion += factor - 1;
        }

        let mut column_expansion = 0;
        for j in 0..grid.ncols() {
            if empty_column_indices.contains(&j) {
                column_expansion += factor - 1;
            }
            if grid[[i, j]] == '#' {
                galaxies.push((i + row_expansion, j + column_expansion));
            }
        }
    }

    Ok(galaxies)
}

fn solution(puzzle_lines: &[String], efactor: usize) -> Result<usize, Box<dyn Error>> {
    let manhatten = |a: (usize, usize), b: (usize, usize)| -> usize {
        ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as usize
    };

    Ok(expanded_galaxies(puzzle_lines, efactor)?
        .iter()
        .combinations(2)
        .map(|pair| manhatten(*pair[0], *pair[1]))
        .sum())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, 2)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, 1000000)
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
        assert_eq!(solution(&puzzle_lines, 2)?, 374);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 9556712);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solution(&puzzle_lines, 10)?, 1030);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solution(&puzzle_lines, 100)?, 8410);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 678626199476);
        Ok(())
    }
}
