use general::{get_args, read_data_lines, reset_sigpipe, trim_split_on};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};
use std::ops::RangeInclusive;

const OFFSETS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

struct Minmax {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

fn get_data(data: &[String]) -> Vec<(i32, i32, i32)> {
    data.iter()
        .map(|line| trim_split_on::<i32>(line, ',').expect("comma separated numbers"))
        .map(|v| (v[0], v[1], v[2]))
        .collect()
}

fn solve1(cubes: &[(i32, i32, i32)]) -> usize {
    let mut faces = 6 * cubes.len();

    for i in 0..cubes.len() {
        let (x1, y1, z1) = cubes[i];
        for (x2, y2, z2) in cubes.iter().skip(i + 1) {
            if (x1, y1) == (*x2, *y2) && (z1 - *z2).abs() == 1
                || (x1, z1) == (*x2, *z2) && (y1 - *y2).abs() == 1
                || (y1, z1) == (*y2, *z2) && (x1 - *x2).abs() == 1
            {
                faces -= 2;
            }
        }
    }
    faces
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let cubes = get_data(puzzle_lines);
    Ok(solve1(&cubes))
}

fn interior(
    pos: (i32, i32, i32),
    cubes: &[(i32, i32, i32)],
    visited: &mut HashSet<(i32, i32, i32)>,
    out: &mut HashSet<(i32, i32, i32)>,
    minmax: &Minmax,
) -> bool {
    if out.contains(&pos) {
        return true;
    }
    if visited.contains(&pos) {
        return false;
    }

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(pos);

    while let Some(c) = q.pop_front() {
        if !cubes.contains(&c) && !seen.contains(&c) {
            let (x, y, z) = c;
            if !minmax.x.contains(&x) && !minmax.y.contains(&y) && !minmax.z.contains(&z) {
                for coord in seen {
                    out.insert(coord);
                }
                return true;
            }
            seen.insert(c);

            for (dx, dy, dz) in OFFSETS {
                q.push_back((x + dx, y + dy, z + dz));
            }
        }
    }

    for coord in seen {
        visited.insert(coord);
    }
    false
}

fn solve2(cubes: &[(i32, i32, i32)]) -> usize {
    let minmax = Minmax {
        x: *cubes.iter().map(|(x, _, _)| x).min().unwrap()
            ..=*cubes.iter().map(|(x, _, _)| x).max().unwrap(),
        y: *cubes.iter().map(|(_, y, _)| y).min().unwrap()
            ..=*cubes.iter().map(|(_, y, _)| y).max().unwrap(),
        z: *cubes.iter().map(|(_, _, z)| z).min().unwrap()
            ..=*cubes.iter().map(|(_, _, z)| z).max().unwrap(),
    };

    let mut out = HashSet::new();
    let mut visited = HashSet::new();

    cubes
        .iter()
        .map(|(x, y, z)| {
            OFFSETS
                .iter()
                .filter(|(dx, dy, dz)| {
                    let pos = (x + dx, y + dy, z + dz);
                    interior(pos, cubes, &mut out, &mut visited, &minmax)
                })
                .count()
        })
        .sum()
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let cubes = get_data(puzzle_lines);
    Ok(solve2(&cubes))
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 64);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 3550);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 58);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2028);
        Ok(())
    }
}
