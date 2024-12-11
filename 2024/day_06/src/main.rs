use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);
type Direction = (isize, isize);

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(
        data.iter().filter(|line| !line.is_empty()).map(|line| line.chars()),
    )?)
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        directions::N => directions::E,
        directions::E => directions::S,
        directions::S => directions::W,
        directions::W => directions::N,
        _ => unreachable!(),
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let mut current = grid.items().find(|((_, _), c)| **c == '^').unwrap().0;

    let mut direction = directions::N;
    let mut points: VecDeque<Point> = grid.in_direction(current, direction).collect();
    let mut visited = HashSet::new();
    visited.insert(current);

    while let Some(point) = points.pop_front() {
        if grid[point] == '#' {
            direction = turn_right(direction);
            points = grid.in_direction(current, direction).collect();
        } else {
            visited.insert(point);
            current = point;
        }
    }

    Ok(visited.len())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let start = grid.items().find(|((_, _), c)| **c == '^').unwrap().0;
    let candidates: Vec<_> = grid.items().filter(|(_, c)| **c == '.').map(|(p, _)| p).collect();

    let mut loop_count = 0;

    for p in candidates {
        let mut current = start;
        let mut direction = directions::N;
        let mut points: VecDeque<Point> = grid.in_direction(current, direction).collect();
        let mut queries = HashSet::new();
        queries.insert((current, direction));

        while let Some(point) = points.pop_front() {
            if point == p || grid[point] == '#' {
                direction = turn_right(direction);
                let query = (current, direction);
                if queries.contains(&query) {
                    loop_count += 1;
                    break;
                }
                queries.insert(query);
                points = grid.in_direction(query.0, query.1).collect();
            } else {
                current = point;
            }
        }
    }

    Ok(loop_count)
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
        assert_eq!(part1(&puzzle_lines)?, 41);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 4656);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 6);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 1575);
        Ok(())
    }
}
