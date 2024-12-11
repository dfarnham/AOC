use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use itertools::Itertools;
use pathfinding::matrix::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(
        data.iter().filter(|line| !line.is_empty()).map(|line| line.chars()),
    )?)
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let antennas: Vec<_> = grid.items().filter(|((_, _), c)| c.is_ascii_alphanumeric()).collect();

    // associate a set of points to each antenna type
    let mut antenna_types: HashMap<char, HashSet<Point>> = HashMap::new();
    for (p, c) in &antennas {
        let set = antenna_types.entry(**c).or_default();
        set.insert(*p);
    }

    let mut antinodes = HashSet::new();
    for set in antenna_types.values().filter(|set| set.len() > 1) {
        for point_pair in set.iter().combinations(2).collect::<Vec<_>>() {
            let run = point_pair[0].0 as i64 - point_pair[1].0 as i64;
            let rise = point_pair[0].1 as i64 - point_pair[1].1 as i64;

            // a: vector of points from point1 opposite the direction of point2
            let a: Vec<_> = grid
                .in_direction(*point_pair[0], (run as isize, rise as isize))
                .collect();

            // b: vector of points from point1 in the direction of point2
            let b: Vec<_> = grid
                .in_direction(*point_pair[0], (-run as isize, -rise as isize))
                .collect();

            if part2 {
                // all the points on the line
                antinodes.insert(*point_pair[0]);
                antinodes.extend(a);
                antinodes.extend(b);
            } else {
                // the first point from point1 opposite the direction of point2
                if !a.is_empty() {
                    antinodes.insert(a[0]);
                }

                // the second point from point1 in the direction of point2
                // (the first point is point2)
                if b.len() > 1 {
                    assert_eq!(*point_pair[1], b[0]);
                    antinodes.insert(b[1]);
                }
            }
        }
    }

    Ok(antinodes.len())
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
        assert_eq!(solve(&puzzle_lines, false)?, 14);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 351);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 34);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, true)?, 9);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 1259);
        Ok(())
    }
}
