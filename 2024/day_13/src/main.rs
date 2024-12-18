use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use num_bigint::BigInt;
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

#[derive(Copy, Clone, Debug)]
struct Behavior {
    a: (usize, usize),
    b: (usize, usize),
    p: (usize, usize),
}

fn get_machines(data: &[String]) -> Result<Vec<Behavior>, Box<dyn Error>> {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    Ok(data
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| {
            let cap1 = re.captures(c[0]).unwrap();
            let cap2 = re.captures(c[1]).unwrap();
            let cap3 = re.captures(c[2]).unwrap();
            Behavior {
                a: (
                    cap1.get(1).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                    cap1.get(2).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                ),
                b: (
                    cap2.get(1).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                    cap2.get(2).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                ),
                p: (
                    cap3.get(1).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                    cap3.get(2).map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                ),
            }
        })
        .collect())
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

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let mut total = BigInt::from(0);

    for machine in get_machines(puzzle_lines)? {
        // coefficient matrix
        let a = [machine.a.0, machine.b.0, machine.a.1, machine.b.1];

        // solution vector
        let b = match part2 {
            true => [machine.p.0 + 10000000000000, machine.p.1 + 10000000000000],
            false => [machine.p.0, machine.p.1],
        };

        // =============
        // Cramer's Rule
        // =============

        // replace columns in "a" with solution vectors for x,y
        let mut bx = a;
        let mut by = a;
        for i in 0..b.len() {
            bx[b.len() * i] = b[i];
            by[b.len() * i + 1] = b[i];
        }

        // prepare the large integer solution using BigInt
        let a: Vec<BigInt> = a.into_iter().map(BigInt::from).collect();
        let bx: Vec<BigInt> = bx.into_iter().map(BigInt::from).collect();
        let by: Vec<BigInt> = by.into_iter().map(BigInt::from).collect();

        // determinant of the coefficient matrix is the denominator in solutions
        let x = det(&bx) / det(&a);
        let y = det(&by) / det(&a);

        // sum valid solutions
        if x.clone() * a[0].clone() + y.clone() * a[1].clone() == b[0].into()
            && x.clone() * a[2].clone() + y.clone() * a[3].clone() == b[1].into()
        {
            total += 3 * x + y;
        }
    }

    // returne the BigInt converted to a usize
    Ok(total.to_string().parse::<usize>()?)
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
        assert_eq!(solve(&puzzle_lines, false)?, 480);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 36250);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 875318608908);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 83232379451012);
        Ok(())
    }
}
