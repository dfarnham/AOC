use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);
type Direction = (isize, isize);

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(
        data.iter()
            .filter(|line| line.starts_with('#'))
            .map(|line| line.chars()),
    )?)
}

// return (direction, cost) of a neighbor (n) given a point (p) and the current direction (d)
fn dir_cost(grid: &Matrix<char>, n: Point, p: Point, d: Direction) -> (Direction, usize) {
    match d == directions::N || d == directions::S {
        true => {
            if grid.move_in_direction(p, directions::W) == Some(n) {
                (directions::W, 1001)
            } else if grid.move_in_direction(p, directions::E) == Some(n) {
                (directions::E, 1001)
            } else {
                (d, 1)
            }
        }
        false => {
            if grid.move_in_direction(p, directions::N) == Some(n) {
                (directions::N, 1001)
            } else if grid.move_in_direction(p, directions::S) == Some(n) {
                (directions::S, 1001)
            } else {
                (d, 1)
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    grid: &Matrix<char>,                              // grid
    d: Direction,                                     // current direction
    s: usize,                                         // current total score
    p: Point,                                         // current point location
    end: Point,                                       // ending location
    best: &mut usize,                                 // known best score
    path_points: &mut HashSet<Point>,                 // current set of points on journey to "end"
    visited: &mut HashMap<(Point, Direction), usize>, // cache
    all_best: &mut HashSet<Point>,                    // result holding all paths with "best" score
) {
    if let Some(score) = visited.get(&(p, d)) {
        if *score < s {
            return;
        }
    }
    visited.insert((p, d), s);

    for n in grid.neighbours(p, false).filter(|p| grid[*p] != '#') {
        let (direction, cost) = dir_cost(grid, n, p, d);
        let score = s + cost;
        if score <= *best {
            if n == end {
                if score < *best {
                    *best = score;
                    all_best.clear();
                }
                all_best.extend(path_points.clone());
            } else if !path_points.contains(&n) {
                path_points.insert(n);
                dfs(grid, direction, score, n, end, best, path_points, visited, all_best);
                path_points.remove(&n);
            }
        }
    }
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let start = grid.items().find(|(_, c)| **c == 'S').unwrap().0;
    let end = grid.items().find(|(_, c)| **c == 'E').unwrap().0;

    let mut best = usize::MAX;
    let mut all_best = HashSet::new();

    dfs(
        &grid,               // grid
        directions::E,       // starting direction
        0,                   // current score
        start,               // starting position
        end,                 // ending position
        &mut best,           // known best score
        &mut HashSet::new(), // contains current path
        &mut HashMap::new(), // cache
        &mut all_best,       // result containing all points in "best" paths (excluding [start, end])
    );

    Ok(if !part2 { best } else { all_best.len() + 2 })
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

    let n = solve(&puzzle_lines, false)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = solve(&puzzle_lines, true)?;
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
        assert_eq!(solve(&puzzle_lines, false)?, 7036);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, false)?, 11048);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 83432);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 45);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, true)?, 64);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 467);
        Ok(())
    }
}
