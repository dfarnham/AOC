use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Array2<char>, Box<dyn Error>> {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), '.');

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }

    Ok(grid)
}

fn score(grid: &Array2<char>) -> usize {
    let mut nrow = grid.nrows();
    let mut cnt = 0;
    for row in grid.rows() {
        cnt += nrow * row.iter().filter(|c| **c == 'O').count();
        nrow -= 1;
    }
    cnt
}

fn tilt_north(grid: &Array2<char>) -> Result<Array2<char>, Box<dyn Error>> {
    let (nrow, ncol) = grid.dim();
    let mut grid = grid.clone();

    for i in 0..nrow {
        for j in 0..ncol {
            if grid[[i, j]] == '.' {
                let mut ii = i;
                while ii < nrow && grid[[ii, j]] == '.' {
                    ii += 1;
                }
                if ii < nrow && grid[[ii, j]] == 'O' {
                    grid[[i, j]] = 'O';
                    grid[[ii, j]] = '.';
                }
            }
        }
    }
    Ok(grid)
}

fn rotate(degrees: i32, tile: &Array2<char>) -> Array2<char> {
    assert_eq!((degrees + 360) % 90, 0, "degrees = {degrees} is not a multiple of 90");
    let mut rotated_tile = tile.clone();
    let dim = tile.nrows();
    let rotation = (degrees + 360) / 90 % 4 * 90;

    if rotation > 0 {
        for i in 0..dim {
            for j in 0..dim {
                rotated_tile[[i, j]] = match (degrees + 360) / 90 % 4 * 90 {
                    90 => tile[[dim - 1 - j, i]],
                    180 => tile[[dim - 1 - i, dim - 1 - j]],
                    270 => tile[[j, dim - 1 - i]],
                    _ => unreachable!(),
                }
            }
        }
    }

    rotated_tile
}

// cycle  n,w,s,e
fn cycle(grid: &Array2<char>) -> Result<Array2<char>, Box<dyn Error>> {
    let mut grid = tilt_north(grid)?;
    for _ in 0..3 {
        grid = rotate(90, &grid);
        grid = tilt_north(&grid)?;
    }
    Ok(rotate(90, &grid))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let grid = tilt_north(&grid)?;
    Ok(score(&grid))
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut grid = get_grid(puzzle_lines)?;
    let mut seen = HashMap::new();

    let target = 1000000000;
    while !seen.contains_key(&grid) {
        seen.insert(grid.clone(), seen.len() + 1);
        grid = cycle(&grid)?;
    }

    let count_in_cycle = seen[&grid];
    let n_before_cycle = seen.len();
    let distance_to_target = target - n_before_cycle;
    let cycle_len = n_before_cycle - count_in_cycle + 1; 
    let n_todo = distance_to_target % cycle_len;

    // cycle to the end
    for _ in 0..n_todo {
        grid = cycle(&grid)?;
    }

    Ok(score(&grid))
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
        assert_eq!(part1(&puzzle_lines)?, 136);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 106648);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 64);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 87700);
        Ok(())
    }
}
