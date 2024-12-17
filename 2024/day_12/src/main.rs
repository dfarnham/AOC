use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use itertools::Itertools;
use pathfinding::matrix::*;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);
const BORDER: char = '.';

fn get_bordered_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    let m = Matrix::from_rows(
        data.iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars())
            .collect::<Vec<_>>(),
    )?;
    let mut g = Matrix::new(m.rows + 2, m.columns + 2, BORDER);
    g.set_slice((1, 1), &m);
    Ok(g)
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let grid = get_bordered_grid(puzzle_lines)?;
    let mut regions: Vec<HashSet<_>> = vec![];
    let mut visited: HashSet<Point> = HashSet::new();

    for (point, plant) in grid.items().filter(|(_, plant)| **plant != BORDER) {
        let mut neighbours: HashSet<_> = grid.neighbours(point, false).filter(|p| grid[*p] == *plant).collect();

        // keep find neighbours of neighbours ...
        let mut sz = 0;
        while sz != neighbours.len() {
            sz = neighbours.len();
            for n in neighbours.clone() {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                neighbours.extend(grid.neighbours(n, false).filter(|p| grid[*p] == *plant));
            }
        }
        neighbours.insert(point);

        // extend an existing region or create a new one
        match regions.iter().position(|region| !neighbours.is_disjoint(region)) {
            Some(i) => regions[i].extend(neighbours),
            None => regions.push(neighbours),
        }
    }

    if !part2 {
        // helper to count the perimeter points
        let perimeter_cnt =
            |p: &Point| -> usize { grid.neighbours(*p, false).filter(|n| grid[*p] != grid[*n]).count() };

        Ok(regions
            .iter()
            .map(|region| region.iter().map(perimeter_cnt).sum::<usize>() * region.len())
            .sum())
    } else {
        // Part 2 works for now but is less than elegant, revisit later
        let mut result = 0;
        for region in &regions {
            let mut sides = 0;
            let perimeter_points = region
                .clone()
                .into_iter()
                .map(|p| {
                    (
                        p,
                        grid.neighbours(p, false)
                            .filter(|n| grid[p] != grid[*n])
                            .collect::<HashSet<_>>(),
                    )
                })
                .filter(|(_, s)| !s.is_empty())
                .collect::<Vec<_>>();

            for pp in perimeter_points.clone().into_iter() {
                if pp.1.len() == 2 {
                    // outside corners
                    let pn = grid.move_in_direction(pp.0, directions::N).unwrap();
                    let pe = grid.move_in_direction(pp.0, directions::E).unwrap();
                    let ps = grid.move_in_direction(pp.0, directions::S).unwrap();
                    let pw = grid.move_in_direction(pp.0, directions::W).unwrap();

                    if (pp.1.contains(&pn) || pp.1.contains(&ps)) && (pp.1.contains(&pe) || pp.1.contains(&pw)) {
                        sides += 1;
                    }
                } else if pp.1.len() == 3 {
                    // dead ends
                    sides += 2;
                } else if pp.1.len() == 4 {
                    // solos
                    sides += 4;
                }
            }

            for combo in perimeter_points.into_iter().combinations(2) {
                let (p1, p2) = (combo[0].0, combo[1].0);
                let intersection = combo[0].1.intersection(&combo[1].1).collect::<Vec<_>>();
                if intersection.len() == 1 {
                    // inside corners
                    let pn = grid.move_in_direction(*intersection[0], directions::N).unwrap();
                    let pe = grid.move_in_direction(*intersection[0], directions::E).unwrap();
                    let ps = grid.move_in_direction(*intersection[0], directions::S).unwrap();
                    let pw = grid.move_in_direction(*intersection[0], directions::W).unwrap();
                    if [pn, ps].contains(&p1) && [pe, pw].contains(&p2)
                        || [pe, pw].contains(&p1) && [pn, ps].contains(&p2)
                    {
                        sides += 1;
                    }
                }
            }

            result += sides * region.len();
        }

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
        assert_eq!(solve(&puzzle_lines, false)?, 140);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, false)?, 772);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(solve(&puzzle_lines, false)?, 1930);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 1477762);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 80);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(solve(&puzzle_lines, true)?, 436);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5")?;
        assert_eq!(solve(&puzzle_lines, true)?, 368);
        Ok(())
    }

    #[test]
    fn part2_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(solve(&puzzle_lines, true)?, 1206);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 923480);
        Ok(())
    }
}
