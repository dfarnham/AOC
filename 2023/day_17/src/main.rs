use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
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

fn jon(graph: &Matrix<usize>, source: Point, target: Point, p2: bool) -> usize {
    let mut priorityq = BTreeSet::new();
    let mut seen = HashSet::new();
    let mut best = usize::MAX;

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let initial_direction = directions.len(); // any value outside the enumeration of directions
    priorityq.insert((0, source.0, source.1, initial_direction, initial_direction));
    while let Some(item) = priorityq.pop_first() {
        let (dist, r, c, dir, indir) = item;
        if (r, c) == target && (indir >= 4 || !p2) {
            best = dist;
            break;
        } else if seen.contains(&(r, c, dir, indir)) {
            continue;
        }
        seen.insert((r, c, dir, indir));

        for (i, p) in directions.iter().enumerate() {
            let (dr, dc) = (p.0, p.1);
            let rr = r as i32 + dr;
            let cc = c as i32 + dc;
            if rr < 0 || rr >= graph.rows as i32 || cc < 0 || cc >= graph.columns as i32 {
                continue;
            }
            let new_dir = i;
            let new_indir = match new_dir != dir {
                true => 1,
                false => indir + 1,
            };
            let isnt_reverse = (new_dir + 2) % 4 != dir;
            if isnt_reverse {
                let (rr, cc) = (rr as usize, cc as usize);
                // Part 1:
                // Because it is difficult to keep the top-heavy crucible going in a straight line
                // for very long, it can move at most three blocks in a single direction before it
                // must turn 90 degrees left or right. The crucible also can't reverse direction;
                // after entering each city block, it may only turn left, continue straight, or
                // turn right
                //
                // Part 2:
                // Once an ultra crucible starts moving in a direction, it needs to move a minimum
                // of four blocks in that direction before it can turn (or even before it can stop
                // at the end). However, it will eventually start to get wobbly: an ultra crucible
                // can move a maximum of ten consecutive blocks without turning.
                let constraint = match p2 {
                    true => indir == initial_direction || new_indir <= 10 && (new_dir == dir || indir >= 4),
                    false => new_indir <= 3,
                };
                if constraint && !seen.contains(&(rr, cc, new_dir, new_indir)) {
                    let cost = graph[(rr, cc)];
                    priorityq.insert((dist + cost, rr, cc, new_dir, new_indir));
                }
            }
        }
    }
    best
}

fn solution(puzzle_lines: &[String], p2: bool) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let s = (0, 0);
    let e = (grid.rows - 1, grid.columns - 1);
    Ok(jon(&grid, s, e, p2))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, false)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, true)
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
        assert_eq!(part1(&puzzle_lines)?, 102);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 1244);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 94);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part2(&puzzle_lines)?, 71);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 1367);
        Ok(())
    }
}
