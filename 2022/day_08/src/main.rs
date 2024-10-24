use general::{get_args, read_data_lines, reset_sigpipe};
use ndarray::*;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Array2<u32> {
    // row parsing rules for lines in data
    const RADIX: u32 = 10;
    let get_row = |s: &str| s.chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), 0);

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }
    grid
}

#[rustfmt::skip]
fn visible_count(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mat = get_grid(puzzle_lines);
    let mut trees = HashSet::new();
    let dim = mat.nrows() - 1;

    for i in 1..dim {
        for j in 1..dim {
            let n = mat[[i, j]];
            if n > *mat.slice(s![i, ..j]).iter().max().unwrap()         // Left
                || n > *mat.slice(s![i, j + 1..]).iter().max().unwrap() // Right
                || n > *mat.slice(s![..i, j]).iter().max().unwrap()     // Up
                || n > *mat.slice(s![i + 1.., j]).iter().max().unwrap() // Down
            {
                trees.insert((i, j));
            }
        }
    }
    Ok(trees.len() + 4 * dim)
}

#[rustfmt::skip]
fn scenic_score(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mat = get_grid(puzzle_lines);
    let dim = mat.nrows() - 1;
    let mut scores = vec![];

    fn count_view(n: u32, vals: &[u32]) -> usize {
        match vals.iter().position(|v| *v >= n) {
            Some(c) => c + 1,
            None => vals.len(),
        }
    }

    for i in 1..dim {
        for j in 1..dim {
            let n = mat[[i, j]];
            let score = 
                count_view(n, &mat.slice(s![i, ..j]).to_vec().iter().rev().copied().collect::<Vec<_>>()) *
                count_view(n, &mat.slice(s![i, j + 1..]).to_vec()) *
                count_view(n, &mat.slice(s![..i, j]).to_vec().iter().rev().copied().collect::<Vec<_>>()) *
                count_view(n, &mat.slice(s![i + 1.., j]).to_vec());
            scores.push(score);
        }
    }
    Ok(*scores.iter().max().unwrap_or(&0))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    visible_count(puzzle_lines)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    scenic_score(puzzle_lines)
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
        assert_eq!(part1(&puzzle_lines)?, 21);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1820);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 8);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 385112);
        Ok(())
    }
}
