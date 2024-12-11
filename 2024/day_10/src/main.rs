use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use pathfinding::prelude::count_paths;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Matrix<i8>, Box<dyn Error>> {
    const RADIX: u32 = 10;
    Ok(Matrix::from_rows(
        data.iter().filter(|line| !line.is_empty()).map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        -1
                    } else {
                        c.to_digit(RADIX).unwrap() as i8
                    }
                })
                .collect::<Vec<_>>()
        }),
    )?)
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let starts: Vec<_> = grid
        .items()
        .filter(|(_, c)| **c == 0)
        .map(|(p, _)| p)
        .collect();
    let ends: Vec<_> = grid
        .items()
        .filter(|(_, c)| **c == 9)
        .map(|(p, _)| p)
        .collect();

    let mut total = 0;
    for s in &starts {
        for e in &ends {
            let num_paths = count_paths(
                *s,
                |p| {
                    let g = grid.clone();
                    let target = g[*p] + 1;
                    g.neighbours(*p, false).filter(move |n| g[*n] == target)
                },
                |p| p == e,
            );

            if part2 {
                total += num_paths;
            } else if num_paths > 0 {
                total += 1;
            }
        }
    }
    Ok(total)
}

#[allow(dead_code)]
fn part1_orig(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grid = get_grid(puzzle_lines)?;
    let starts: Vec<_> = grid
        .items()
        .filter(|(_, c)| **c == 0)
        .map(|(p, _)| p)
        .collect();
    let ends: Vec<_> = grid
        .items()
        .filter(|(_, c)| **c == 9)
        .map(|(p, _)| p)
        .collect();

    let mut total = 0;
    for s in &starts {
        for e in &ends {
            let mut workq = VecDeque::new();
            workq.push_back(*s);
            while let Some(p) = workq.pop_front() {
                if p == *e {
                    total += 1;
                    break;
                }
                for neighbor in grid.neighbours(p, false) {
                    if grid[p] + 1 == grid[neighbor] {
                        workq.push_back(neighbor)
                    }
                }
            }
        }
    }
    Ok(total)
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
        assert_eq!(solve(&puzzle_lines, false)?, 1);
        assert_eq!(part1_orig(&puzzle_lines)?, 1);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, false)?, 2);
        assert_eq!(part1_orig(&puzzle_lines)?, 2);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 733);
        assert_eq!(part1_orig(&puzzle_lines)?, 733);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(solve(&puzzle_lines, true)?, 3);
        Ok(())
    }

    #[test]
    fn part2_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4")?;
        assert_eq!(solve(&puzzle_lines, true)?, 13);
        Ok(())
    }

    #[test]
    fn part2_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5")?;
        assert_eq!(solve(&puzzle_lines, true)?, 81);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 1514);
        Ok(())
    }
}
