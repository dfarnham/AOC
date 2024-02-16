use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn lines_intersect(line1: Line, line2: Line) -> bool {
    line1.start.x.max(line2.start.x) <= line1.end.x.min(line2.end.x)
        && line1.start.y.max(line2.start.y) <= line1.end.y.min(line2.end.y)
}

fn get_bricks(puzzle_lines: &[String]) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut bricks = vec![];
    for line in puzzle_lines {
        let xyz = trim_split_on::<usize>(&line.replace('~', ","), ',')?;
        bricks.push(Line {
            start: Point {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            },
            end: Point {
                x: xyz[3],
                y: xyz[4],
                z: xyz[5],
            },
        });
    }
    bricks.sort_by(|line1, line2| line1.start.z.cmp(&line2.start.z));
    Ok(bricks)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut bricks = get_bricks(puzzle_lines)?;

    for i in 0..bricks.len() {
        let z = bricks[0..i]
            .iter()
            .filter(|other| lines_intersect(bricks[i], **other))
            .fold(1, |maxz, other| maxz.max(other.end.z + 1));
        /*
        let mut z = 1;
        for other in &bricks[0..i] {
            if lines_intersect(bricks[i], *other) {
                z = z.max(other.end.z + 1);
            }
        }
        */
        bricks[i].end.z -= bricks[i].start.z - z;
        bricks[i].start.z = z;
    }

    //bricks.sort_by(|line1, line2| line1.start.z.cmp(&line2.start.z));

    let mut k_v = vec![HashSet::new(); bricks.len()];
    let mut v_k = vec![HashSet::new(); bricks.len()];

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[0..j].iter().enumerate() {
            if lines_intersect(*lower, *upper) && upper.start.z == lower.end.z + 1 {
                k_v[i].insert(j);
                v_k[j].insert(i);
            }
        }
    }

    Ok(k_v.iter().filter(|set| set.iter().all(|j| v_k[*j].len() > 1)).count())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut bricks = get_bricks(puzzle_lines)?;

    for i in 0..bricks.len() {
        let z = bricks[0..i]
            .iter()
            .filter(|other| lines_intersect(bricks[i], **other))
            .fold(1, |maxz, other| maxz.max(other.end.z + 1));
        bricks[i].end.z -= bricks[i].start.z - z;
        bricks[i].start.z = z;
    }

    let mut k_v = vec![HashSet::new(); bricks.len()];
    let mut v_k = vec![HashSet::new(); bricks.len()];

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[0..j].iter().enumerate() {
            if lines_intersect(*lower, *upper) && upper.start.z == lower.end.z + 1 {
                k_v[i].insert(j);
                v_k[j].insert(i);
            }
        }
    }

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut workq = k_v[i].iter().filter(|j| v_k[**j].len() == 1).copied().collect::<VecDeque<_>>();
        let mut falling: HashSet::<usize> = HashSet::from_iter(workq.clone());
        falling.insert(i);

        while let Some(j) = workq.pop_front() {
            for k in &k_v[j] {
                if !falling.contains(k) && v_k[*k].iter().all(|s| falling.contains(s)) {
                    workq.push_back(*k);
                    falling.insert(*k);
                }
            }
        }
        total += falling.len() - 1;
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
        assert_eq!(part1(&puzzle_lines)?, 5);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 426);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 7);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 61920);
        Ok(())
    }
}
