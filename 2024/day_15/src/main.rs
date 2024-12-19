use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Write};

type Direction = (isize, isize);
type Point = (usize, usize);

#[allow(clippy::type_complexity)]
fn get_grid_moves(data: &[String]) -> Result<(Matrix<char>, Point, Vec<Direction>), Box<dyn Error>> {
    let grid = Matrix::from_rows(
        data.iter()
            .filter(|line| line.starts_with('#'))
            .map(|line| line.chars()),
    )?;

    let robot = grid.items().find(|(_, c)| **c == '@').unwrap().0;

    let moves: Vec<_> = data
        .join("")
        .chars()
        .filter(|c| ['^', 'v', '<', '>'].contains(c))
        .map(|c| match c {
            '^' => directions::N,
            'v' => directions::S,
            '<' => directions::W,
            '>' => directions::E,
            _ => unreachable!(),
        })
        .collect();

    Ok((grid, robot, moves))
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mut grid, mut robot, moves) = get_grid_moves(puzzle_lines)?;

    for m in moves {
        if let Some(p) = grid.move_in_direction(robot, m) {
            if grid[p] == '.' {
                grid[robot] = '.';
                robot = p;
                grid[robot] = '@';
            } else if grid[p] == 'O' {
                let mut t = p;
                while let Some(pp) = grid.move_in_direction(t, m) {
                    if grid[pp] == 'O' {
                        t = pp;
                    } else {
                        if grid[pp] == '.' {
                            grid[robot] = '.';
                            robot = p;
                            grid[robot] = '@';
                            grid[pp] = 'O';
                        }
                        break;
                    }
                }
            }
        }
    }

    Ok(grid
        .items()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| 100 * p.0 + p.1)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let puzzle_lines = puzzle_lines
        .iter()
        .map(|line| {
            line.replace("#", "##")
                .replace("O", "[]")
                .replace(".", "..")
                .replace("@", "@.")
                .to_string()
        })
        .collect::<Vec<_>>();
    let (mut grid, mut robot, moves) = get_grid_moves(&puzzle_lines)?;

    #[allow(unused_variables)]
    let display = |g: Matrix<char>| {
        for row in g.iter() {
            for elem in row {
                print!("{elem}");
            }
            println!();
        }
    };

    for m in moves {
        if let Some(p) = grid.move_in_direction(robot, m) {
            if grid[p] == '.' {
                grid[robot] = '.';
                robot = p;
                grid[robot] = '@';
                //display(grid.clone());
            } else if grid[p] == '[' || grid[p] == ']' {
                let bx = if grid[p] == '[' {
                    (p, grid.move_in_direction(p, directions::E).unwrap())
                } else {
                    (grid.move_in_direction(p, directions::W).unwrap(), p)
                };

                let mut success = true;
                let mut to_move = vec![];
                let mut workq = VecDeque::new();
                to_move.push(bx);
                workq.push_back(bx);
                // find all the boxes that need to be moved
                while let Some(bx) = workq.pop_front() {
                    let (l, r) = (bx.0, bx.1);
                    assert_eq!(grid[l], '[');
                    assert_eq!(grid[r], ']');

                    let p1 = grid.move_in_direction(l, m).unwrap();
                    let p2 = grid.move_in_direction(r, m).unwrap();
                    if grid[p1] == '#' || grid[p2] == '#' {
                        success = false;
                        break;
                    }

                    match m {
                        directions::N | directions::S => {
                            if grid[p1] == '[' {
                                if !to_move.contains(&(p1, p2)) {
                                    workq.push_back((p1, p2));
                                    to_move.push((p1, p2));
                                }
                            } else {
                                if grid[p1] == ']' {
                                    let ll = grid.move_in_direction(p1, directions::W).unwrap();
                                    if !to_move.contains(&(ll, p1)) {
                                        workq.push_back((ll, p1));
                                        to_move.push((ll, p1));
                                    }
                                }
                                if grid[p2] == '[' {
                                    let rr = grid.move_in_direction(p2, directions::E).unwrap();
                                    if !to_move.contains(&(p2, rr)) {
                                        workq.push_back((p2, rr));
                                        to_move.push((p2, rr));
                                    }
                                }
                            }
                        }
                        directions::W => {
                            if grid[p1] == ']' {
                                let ll = grid.move_in_direction(p1, m).unwrap();
                                assert_eq!(grid[ll], '[');
                                if !to_move.contains(&(ll, p1)) {
                                    workq.push_back((ll, p1));
                                    to_move.push((ll, p1));
                                }
                            }
                        }
                        directions::E => {
                            if grid[p2] == '[' {
                                let rr = grid.move_in_direction(p2, m).unwrap();
                                assert_eq!(grid[rr], ']');
                                if !to_move.contains(&(p2, rr)) {
                                    workq.push_back((p2, rr));
                                    to_move.push((p2, rr));
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if success {
                    while let Some(bx) = to_move.pop() {
                        let (l, r) = (bx.0, bx.1);
                        let (c1, c2) = (grid[l], grid[r]);
                        let p1 = grid.move_in_direction(l, m).unwrap();
                        let p2 = grid.move_in_direction(r, m).unwrap();
                        if m != directions::W {
                            grid[l] = '.';
                        }
                        if m != directions::E {
                            grid[r] = '.';
                        }
                        grid[p1] = c1;
                        grid[p2] = c2;
                    }
                    grid[robot] = '.';
                    robot = p;
                    grid[robot] = '@';
                }
            }
            //display(grid.clone());
        }
    }

    Ok(grid
        .items()
        .filter(|(_, c)| **c == '[')
        .map(|(p, _)| 100 * p.0 + p.1)
        .sum())
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
        assert_eq!(part1(&puzzle_lines)?, 2028);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part1(&puzzle_lines)?, 10092);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(part1(&puzzle_lines)?, 908);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 1486930);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 1751);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part2(&puzzle_lines)?, 9021);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(part2(&puzzle_lines)?, 618);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 1492011);
        Ok(())
    }
}
