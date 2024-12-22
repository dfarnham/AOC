use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use pathfinding::matrix::*;
use pathfinding::prelude::dijkstra;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);

fn get_data(data: &[String]) -> Result<Vec<Point>, Box<dyn Error>> {
    Ok(data
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| trim_split_on::<usize>(line, ',').unwrap())
        .map(|coordinates| (coordinates[0], coordinates[1]))
        .collect())
}

// testing out pathfinding::prelude::dijkstra
fn dijkstra_alg(grid: &Matrix<bool>) -> Option<usize> {
    let s = (0, 0);
    let e = (grid.rows - 1, grid.columns - 1);

    // dijkstra() requires each neighbor be paired with a cost.
    // the cost for our data is constant
    //
    // cost mapping helper/iterator
    let neighbor_cost = |p: &Point| grid.neighbours(*p, false).filter(|n| grid[*n]).map(|p| (p, 1));

    let result = dijkstra(
        // starting position
        &s,
        //
        // retrieve the (neighbor, cost) relative to 'p'
        |p: &Point| neighbor_cost(p),
        //
        // stopping condition
        |p| *p == e,
    );

    match result {
        Some(res) => Some(res.1),
        _ => None,
    }
}

fn shortest_path(grid: &Matrix<bool>) -> Option<usize> {
    let (start, end) = ((0, 0), (grid.rows - 1, grid.columns - 1));
    let mut workq = VecDeque::new();
    let mut visited = HashSet::new();
    workq.push_back((start, 0));
    while let Some((p, d)) = workq.pop_front() {
        if !visited.contains(&p) {
            visited.insert(p);

            // when found return the distance
            if p == end {
                return Some(d);
            }

            // add neighbors to the work queue, adding +1 to their distance
            for neighbor in grid.neighbours(p, false).filter(|n| grid[*n]) {
                workq.push_back((neighbor, d + 1));
            }
        }
    }
    None
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<String, Box<dyn Error>> {
    let points = get_data(puzzle_lines)?;
    let mut result = "".to_string();

    let mut grid = Matrix::new(71, 71, true);
    for p in points.iter().take(1024) {
        grid[*p] = false;
    }

    if !part2 {
        let steps = dijkstra_alg(&grid);
        assert_eq!(steps, shortest_path(&grid));
        result = steps.unwrap().to_string();
    } else {
        // start from 1024 and go until shortest_path() or dijkstra_alg() fails
        for p in &points[1024..] {
            grid[*p] = false;
            //if dijkstra_alg(&grid).is_none() {
            if shortest_path(&grid).is_none() {
                result = format!("{},{}", p.0, p.1).to_string();
                break;
            }
        }
    }
    Ok(result)
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
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, "304".to_string());
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, "50,28".to_string());
        Ok(())
    }
}
