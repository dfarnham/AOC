use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use itertools::Itertools;
use num_bigint::BigInt;
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
    slope: f64,
    intercept: f64,
}

fn get_lines2d(puzzle_lines: &[String]) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines = vec![];
    for s in puzzle_lines {
        let xyz = trim_split_on::<i64>(&s.replace('@', ","), ',')?;
        // use the first point and the trajectories to compute the next point
        let start = Point { x: xyz[0], y: xyz[1] };
        let end = Point {
            x: xyz[0] + xyz[3],
            y: xyz[1] + xyz[4],
        };

        // compute the slope and intercept; all points have a trajectory so no divisions by zero
        let slope = (end.y - start.y) as f64 / (end.x - start.x) as f64;
        let intercept = end.y as f64 - slope * end.x as f64;

        lines.push(Line {
            start,
            end,
            slope,
            intercept,
        });
    }

    Ok(lines)
}

fn part1(puzzle_lines: &[String], w: usize, h: usize) -> Result<usize, Box<dyn Error>> {
    // lines is a list of Line structs
    let lines = get_lines2d(puzzle_lines)?;

    // the bounding box to look for intersections
    let w = w as f64;
    let h = h as f64;

    // define an intersect closure which returns the intersection point of 2 lines
    let intersect = |line1: &Line, line2: &Line| -> (f64, f64) {
        let x = (line2.intercept - line1.intercept) / (line1.slope - line2.slope);
        let y = line2.slope * x + line2.intercept;
        (x, y)
    };

    // count the intersections between all pairs within the bounding box
    Ok(lines
        .iter()
        .combinations(2)
        .collect::<Vec<_>>()
        .iter()
        .map(|line_pair| (line_pair[0], line_pair[1]))
        .map(|(line1, line2)| (line1, line2, intersect(line1, line2)))
        .filter(|(line1, line2, pt)| {
            (pt.0 >= w && pt.0 <= h && pt.1 >= w && pt.1 <= h)
                && (line1.start.x > line1.end.x && pt.0 <= line1.start.x as f64
                    || line1.start.x < line1.end.x && pt.0 >= line1.start.x as f64)
                && (line2.start.x > line2.end.x && pt.0 <= line2.start.x as f64
                    || line2.start.x < line2.end.x && pt.0 >= line2.start.x as f64)
        })
        .count())
}

// matrix determinant using BigInt
fn det(mat: &[BigInt]) -> BigInt {
    let mut sz = 2;
    while sz * sz != mat.len() {
        sz += 1;
    }

    if sz == 2 {
        mat[0].clone() * mat[3].clone() - mat[1].clone() * mat[2].clone()
    } else {
        let mut result = BigInt::from(0);
        let mut sign = 1;
        for n in 0..sz {
            let mut v = vec![];
            for i in 1..sz {
                for j in (0..sz).filter(|j| *j != n) {
                    v.push(mat[i * sz + j].clone())
                }
            }
            result += sign * mat[n].clone() * det(&v);
            sign = -sign;
        }
        result
    }
}

#[rustfmt::skip]
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut hailstones = vec![];

    // any 3 points provide an integer solution (current algorithm)
    //
    // I originally looped over all stones taking 3 at a time, hashing
    // solver results and using the first duplicate answer to get
    // around floating point differences and that works fine too
    // for those class of solvers but this feels a little cleaner
    for s in &puzzle_lines[0..3] {
        let parse = trim_split_on::<i64>(&s.replace('@', ","), ',')?;
        hailstones.push((parse[0], parse[1], parse[2], parse[3], parse[4], parse[5]));
    }

    // points and velocities
    let p1 = [hailstones[0].0, hailstones[0].1, hailstones[0].2];
    let v1 = [hailstones[0].3, hailstones[0].4, hailstones[0].5];

    let p2 = [hailstones[1].0, hailstones[1].1, hailstones[1].2];
    let v2 = [hailstones[1].3, hailstones[1].4, hailstones[1].5];

    let p3 = [hailstones[2].0, hailstones[2].1, hailstones[2].2];
    let v3 = [hailstones[2].3, hailstones[2].4, hailstones[2].5];

    // coefficient matrix
    let a = [
        v2[1] - v1[1] , v1[0] - v2[0] , 0             , p1[1] - p2[1] , p2[0] - p1[0] , 0             ,
        v3[1] - v1[1] , v1[0] - v3[0] , 0             , p1[1] - p3[1] , p3[0] - p1[0] , 0             ,
        0             , v2[2] - v1[2] , v1[1] - v2[1] , 0             , p1[2] - p2[2] , p2[1] - p1[1] ,
        0             , v3[2] - v1[2] , v1[1] - v3[1] , 0             , p1[2] - p3[2] , p3[1] - p1[1] ,
        v2[2] - v1[2] , 0             , v1[0] - v2[0] , p1[2] - p2[2] , 0             , p2[0] - p1[0] ,
        v3[2] - v1[2] , 0             , v1[0] - v3[0] , p1[2] - p3[2] , 0             , p3[0] - p1[0] ,
    ];

    // solution vector
    let b = [
        (p1[1] * v1[0] - p2[1] * v2[0]) - (p1[0] * v1[1] - p2[0] * v2[1]),
        (p1[1] * v1[0] - p3[1] * v3[0]) - (p1[0] * v1[1] - p3[0] * v3[1]),
        (p1[2] * v1[1] - p2[2] * v2[1]) - (p1[1] * v1[2] - p2[1] * v2[2]),
        (p1[2] * v1[1] - p3[2] * v3[1]) - (p1[1] * v1[2] - p3[1] * v3[2]),
        (p1[2] * v1[0] - p2[2] * v2[0]) - (p1[0] * v1[2] - p2[0] * v2[2]),
        (p1[2] * v1[0] - p3[2] * v3[0]) - (p1[0] * v1[2] - p3[0] * v3[2]),
    ];

    // =============
    // Cramer's Rule
    // =============

    // replace columns in "a" with solution vectors for x,y,z
    let mut bx = a;
    let mut by = a;
    let mut bz = a;
    for i in 0..b.len() {
        bx[b.len() * i] = b[i];
        by[b.len() * i + 1] = b[i];
        bz[b.len() * i + 2] = b[i];
    }

    // prepare the large integer solution using BigInt
    let a: Vec<BigInt> = a.into_iter().map(BigInt::from).collect();
    let bx: Vec<BigInt> = bx.into_iter().map(BigInt::from).collect();
    let by: Vec<BigInt> = by.into_iter().map(BigInt::from).collect();
    let bz: Vec<BigInt> = bz.into_iter().map(BigInt::from).collect();
    
    // determinant of the coefficient matrix is the denominator in solutions
    let answer = (det(&bx) + det(&by) + det(&bz)) / det(&a);
    Ok(answer.to_u64_digits().1[0] as usize)
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

    if puzzle_lines.len() == 5 {
        writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines, 7, 27)?)?;
    } else {
        writeln!(
            stdout,
            "Answer Part 1 = {:?}",
            part1(&puzzle_lines, 200000000000000, 400000000000000)?
        )?;
    }
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines)?)?;

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
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part1(&puzzle_lines, 7, 27)?, 2);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines, 200000000000000, 400000000000000)?, 21785);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 47);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 554668916217145);
        Ok(())
    }
}
