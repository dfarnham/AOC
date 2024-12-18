use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::prelude::Grid;
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    start: Point,
    velocity: Point,
}

fn get_robots(data: &[String]) -> Result<Vec<Robot>, Box<dyn Error>> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    Ok(data
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let cap = re.captures(s).unwrap();
            Robot {
                start: Point {
                    x: cap.get(1).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
                    y: cap.get(2).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
                },
                velocity: Point {
                    x: cap.get(3).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
                    y: cap.get(4).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
                },
            }
        })
        .collect())
}

fn solve(puzzle_lines: &[String], width: usize, height: usize, part2: bool) -> Result<usize, Box<dyn Error>> {
    let width = width as i64;
    let height = height as i64;
    let robots = get_robots(puzzle_lines)?;

    let advance = |p: Point, v: Point| -> Point {
        Point {
            x: match p.x + v.x {
                n if n < 0 => width + n,
                n if n < width => n,
                n => n - width,
            },
            y: match p.y + v.y {
                n if n < 0 => height + n,
                n if n < height => n,
                n => n - height,
            },
        }
    };

    if !part2 {
        let mut quads = [0; 4];
        for robot in &robots {
            let mut pos = robot.start;
            for _ in 0..100 {
                pos = advance(pos, robot.velocity);
            }
            if pos.x < width / 2 && pos.y < height / 2 {
                quads[0] += 1;
            } else if pos.x < width / 2 && pos.y > height / 2 {
                quads[1] += 1;
            } else if pos.x > width / 2 && pos.y < height / 2 {
                quads[2] += 1;
            } else if pos.x > width / 2 && pos.y > height / 2 {
                quads[3] += 1;
            }
        }
        Ok(quads.iter().product())
    } else {
        let mut points: Vec<_> = robots.iter().enumerate().map(|(i, r)| (i, r.start)).collect();
        let mut best = 0;
        let mut result = 0;
        let mut g = Grid::new(0, 0);
        for iteration in 1..10000 {
            points = points
                .into_iter()
                .map(|(i, p)| (i, advance(p, robots[i].velocity)))
                .collect();

            // sum the frequency of the most populated row and column
            let heuristic = points
                .clone()
                .into_iter()
                .map(|(_, p)| p.x as usize)
                .collect::<Counter<_>>()
                .values()
                .max()
                .unwrap()
                + points
                    .clone()
                    .into_iter()
                    .map(|(_, p)| p.y as usize)
                    .collect::<Counter<_>>()
                    .values()
                    .max()
                    .unwrap();

            if heuristic > best {
                best = heuristic;
                result = iteration;
                g = points
                    .clone()
                    .into_iter()
                    .map(|(_, p)| (p.x as usize, p.y as usize))
                    .collect::<Grid>();
            }
        }
        println!("{g:?}\n{best}\n");
        Ok(result)
    }
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

    let n = solve(&puzzle_lines, 101, 103, false)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    if puzzle_lines.len() > 499 {
        let n = solve(&puzzle_lines, 101, 103, true)?;
        writeln!(stdout, "Answer Part 2 = {n}")?;
    }

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
        assert_eq!(solve(&puzzle_lines, 11, 7, false)?, 12);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, 101, 103, false)?, 228690000);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, 101, 103, true)?, 7093);
        Ok(())
    }
}
