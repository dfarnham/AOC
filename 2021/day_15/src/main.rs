use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use pathfinding::prelude::dijkstra_partial;
use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);

fn row_values(s: &str) -> Vec<usize> {
    s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn get_grid(data: &[String]) -> Result<Matrix<usize>, Box<dyn Error>> {
    Ok(Matrix::from_rows(data.iter().map(|line| row_values(line)))?)
}

fn get_grid_x5(data: &[String]) -> Result<Matrix<usize>, Box<dyn Error>> {
    let extend_row_right = |row: &[usize]| -> Vec<usize> {
        let mut erow = row.to_vec();
        for i in 0..4 {
            erow.extend(row.iter().map(|n| (*n + i) % 9 + 1).collect::<Vec<_>>());
        }
        erow
    };

    let mut v = vec![];
    for i in 0..5 {
        for line in data {
            let mut row = extend_row_right(&row_values(line));
            for _ in 0..i {
                row = row.iter().map(|n| *n % 9 + 1).collect();
            }
            v.extend(row);
        }
    }

    Ok(Matrix::from_vec(data.len() * 5, data[0].len() * 5, v)?)
}

#[allow(dead_code)]
fn solution2(graph: &Matrix<usize>) -> usize {
    let s = (0, 0);
    let e = (graph.rows - 1, graph.columns - 1);
    let mut priorityq = BTreeSet::new();
    let mut seen = HashSet::new();
    let mut score = usize::MAX;

    priorityq.insert((0, s.0, s.1));
    while let Some(item) = priorityq.pop_first() {
        let (dist, r, c) = item;
        if (r, c) == e {
            score = dist;
            break;
        } else if seen.contains(&(r, c)) {
            continue;
        }
        seen.insert((r, c));

        for (i, j) in graph.neighbours((r, c), false).filter(|p| !seen.contains(p)) {
            let cost = graph[(i, j)];
            priorityq.insert((dist + cost, i, j));
        }
    }
    score
}

fn solution(grid: &Matrix<usize>) -> usize {
    let s = (0, 0);
    let e = (grid.rows - 1, grid.columns - 1);

    // dijkstra_partial() requires each neighbor be paired with a cost.
    // the cost for our data is the grid value at that point
    //
    // cost mapping helper/iterator for neighbors (diagonals = false)
    let neighbor_cost = |p: &Point| grid.neighbours(*p, false).map(|p| (p, grid[p]));

    let result = dijkstra_partial(
        // starting position
        &s,
        //
        // retrieve (neighbor, cost) relative to 'p'
        |p: &Point| neighbor_cost(p),
        //
        // stopping condition
        |p: &Point| *p == e,
    );

    // result is a tuple: (HashMap<Point, (Point, Cost)>, Option<Point>)
    //
    // https://docs.rs/pathfinding/4.8.0/pathfinding/directed/dijkstra/fn.dijkstra_partial.html
    // The result is a map where every node examined before the algorithm
    // stopped (not including start) is associated with an optimal parent
    // node and a cost from the start node, as well as the node which caused
    // the algorithm to stop if any.
    result.0[&e].1
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_trimmed_data_lines::<String>(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    let grid = get_grid(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 1 = {}", solution(&grid))?;

    let grid = get_grid_x5(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 2 = {}", solution(&grid))?;

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
        Ok(read_trimmed_data_lines::<String>(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example")?;
        let grid = get_grid(&data)?;
        assert_eq!(solution(&grid), 40);
        assert_eq!(solution2(&grid), 40);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid(&data)?;
        assert_eq!(solution(&grid), 540);
        assert_eq!(solution2(&grid), 540);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example")?;
        let grid = get_grid_x5(&data)?;
        assert_eq!(solution(&grid), 315);
        assert_eq!(solution2(&grid), 315);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid_x5(&data)?;
        assert_eq!(solution(&grid), 2879);
        assert_eq!(solution2(&grid), 2879);
        Ok(())
    }
}
