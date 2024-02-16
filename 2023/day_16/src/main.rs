use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

const UP: (i64, i64) = (-1, 0);
const DOWN: (i64, i64) = (1, 0);
const LEFT: (i64, i64) = (0, -1);
const RIGHT: (i64, i64) = (0, 1);

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

fn new_point(dim: (usize, usize), point: (usize, usize), direction: (i64, i64)) -> Option<(usize, usize)> {
    let (nrow, ncol) = dim;
    let (i, j) = (point.0 as i64 + direction.0, point.1 as i64 + direction.1);
    match i >= 0 && j >= 0 && i < nrow as i64 && j < ncol as i64 {
        true => Some((i as usize, j as usize)),
        false => None,
    }
}

fn get_beams(grid: &Array2<char>, beam: ((usize, usize), (i64, i64))) -> Vec<((usize, usize), (i64, i64))> {
    let current = beam.0;
    let direction = beam.1;

    let mut new_beams = vec![];

    // two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -)
    match grid[[current.0, current.1]] {
        '.' => {
            if let Some(point) = new_point(grid.dim(), current, direction) {
                new_beams.push((point, direction));
            }
        }
        '/' => match direction {
            UP => {
                if let Some(point) = new_point(grid.dim(), current, RIGHT) {
                    new_beams.push((point, RIGHT));
                }
            }
            DOWN => {
                if let Some(point) = new_point(grid.dim(), current, LEFT) {
                    new_beams.push((point, LEFT));
                }
            }

            LEFT => {
                if let Some(point) = new_point(grid.dim(), current, DOWN) {
                    new_beams.push((point, DOWN));
                }
            }
            RIGHT => {
                if let Some(point) = new_point(grid.dim(), current, UP) {
                    new_beams.push((point, UP));
                }
            }
            _ => unreachable!(),
        },
        '\\' => match direction {
            UP => {
                if let Some(point) = new_point(grid.dim(), current, LEFT) {
                    new_beams.push((point, LEFT));
                }
            }
            DOWN => {
                if let Some(point) = new_point(grid.dim(), current, RIGHT) {
                    new_beams.push((point, RIGHT));
                }
            }

            LEFT => {
                if let Some(point) = new_point(grid.dim(), current, UP) {
                    new_beams.push((point, UP));
                }
            }
            RIGHT => {
                if let Some(point) = new_point(grid.dim(), current, DOWN) {
                    new_beams.push((point, DOWN));
                }
            }
            _ => unreachable!(),
        },
        '|' => match direction {
            UP | DOWN => {
                if let Some(point) = new_point(grid.dim(), current, direction) {
                    new_beams.push((point, direction));
                }
            }
            LEFT | RIGHT => {
                if let Some(point) = new_point(grid.dim(), current, UP) {
                    new_beams.push((point, UP));
                }
                if let Some(point) = new_point(grid.dim(), current, DOWN) {
                    new_beams.push((point, DOWN));
                }
            }
            _ => unreachable!(),
        },
        '-' => match direction {
            UP | DOWN => {
                if let Some(point) = new_point(grid.dim(), current, LEFT) {
                    new_beams.push((point, LEFT));
                }
                if let Some(point) = new_point(grid.dim(), current, RIGHT) {
                    new_beams.push((point, RIGHT));
                }
            }
            LEFT | RIGHT => {
                if let Some(point) = new_point(grid.dim(), current, direction) {
                    new_beams.push((point, direction));
                }
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    new_beams
}

fn get_energized_count(grid: &Array2<char>, point: (usize, usize), direction: (i64, i64)) -> usize {
    let mut workq = VecDeque::new();
    let mut energized = HashSet::new();

    workq.push_back((point, direction));
    while let Some(beam) = workq.pop_front() {
        if !energized.contains(&beam) {
            energized.insert(beam);
            for new_beam in get_beams(grid, beam) {
                workq.push_back(new_beam);
            }
        }
    }

    // strip the direction from each beam and count occurrences
    energized
        .iter()
        .map(|directional_beam| directional_beam.0)
        .collect::<HashSet<_>>()
        .len()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    Ok(get_energized_count(&grid, (0, 0), RIGHT))
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let (nrow, ncol) = grid.dim();

    let mut best = 0;
    for i in 0..nrow {
        best = best.max(get_energized_count(&grid, (i, 0), RIGHT));
        best = best.max(get_energized_count(&grid, (i, ncol - 1), LEFT));
    }
    for i in 0..ncol {
        best = best.max(get_energized_count(&grid, (0, i), DOWN));
        best = best.max(get_energized_count(&grid, (nrow - 1, i), UP));
    }
    Ok(best)
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
        assert_eq!(part1(&puzzle_lines)?, 46);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 7185);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 51);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 7616);
        Ok(())
    }
}
