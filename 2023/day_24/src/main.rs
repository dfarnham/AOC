use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use itertools::Itertools;
use mathru::{
    algebra::linear::{
        matrix::{General, Solve, Transpose},
        vector::Vector,
    },
    matrix, vector,
};
use num_bigint::BigInt;
use std::error::Error;
use std::io::{self, Write};

/*
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};
*/

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

/*
#[allow(dead_code)]
fn part2_z3_solver(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut hail = vec![];
    for s in &puzzle_lines[0..3] {
        let parse = trim_split_on::<i64>(&s.replace('@', ","), ',')?;
        hail.push((parse[0], parse[1], parse[2], parse[3], parse[4], parse[5]));
    }

    // create a new solver
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // points and velocities from the input
    let (p1_x, p1_y, p1_z, v1_x, v1_y, v1_z) = (
        Int::from_i64(&ctx, hail[0].0),
        Int::from_i64(&ctx, hail[0].1),
        Int::from_i64(&ctx, hail[0].2),
        Int::from_i64(&ctx, hail[0].3),
        Int::from_i64(&ctx, hail[0].4),
        Int::from_i64(&ctx, hail[0].5),
    );

    let (p2_x, p2_y, p2_z, v2_x, v2_y, v2_z) = (
        Int::from_i64(&ctx, hail[1].0),
        Int::from_i64(&ctx, hail[1].1),
        Int::from_i64(&ctx, hail[1].2),
        Int::from_i64(&ctx, hail[1].3),
        Int::from_i64(&ctx, hail[1].4),
        Int::from_i64(&ctx, hail[1].5),
    );

    let (p3_x, p3_y, p3_z, v3_x, v3_y, v3_z) = (
        Int::from_i64(&ctx, hail[2].0),
        Int::from_i64(&ctx, hail[2].1),
        Int::from_i64(&ctx, hail[2].2),
        Int::from_i64(&ctx, hail[2].3),
        Int::from_i64(&ctx, hail[2].4),
        Int::from_i64(&ctx, hail[2].5),
    );

    // create labels to solve for
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");
    let t1 = Int::new_const(&ctx, "t1");
    let t2 = Int::new_const(&ctx, "t2");
    let t3 = Int::new_const(&ctx, "t3");

    // Solver assertions -- 6 equations with 6 unknowns
    //let zero = Int::from_i64(&ctx, 0);
    //solver.assert(&(&(&x - &p1_x) * &(&v1_y - &vy) - &(&y - &p1_y) * &(&v1_x - &vx))._eq(&zero));
    //solver.assert(&(&(&y - &p1_y) * &(&v1_z - &vz) - &(&z - &p1_z) * &(&v1_y - &vy))._eq(&zero));

    //solver.assert(&(&(&x - &p2_x) * &(&v2_y - &vy) - &(&y - &p2_y) * &(&v2_x - &vx))._eq(&zero));
    //solver.assert(&(&(&y - &p2_y) * &(&v2_z - &vz) - &(&z - &p2_z) * &(&v2_y - &vy))._eq(&zero));

    //solver.assert(&(&(&x - &p3_x) * &(&v3_y - &vy) - &(&y - &p3_y) * &(&v3_x - &vx))._eq(&zero));
    //solver.assert(&(&(&y - &p3_y) * &(&v3_z - &vz) - &(&z - &p3_z) * &(&v3_y - &vy))._eq(&zero));

    // Solver assertions -- 9 unknowns (adding time)
    solver.assert(&(&x + &t1 * &vx - &t1 * v1_x)._eq(&p1_x));
    solver.assert(&(&y + &t1 * &vy - &t1 * v1_y)._eq(&p1_y));
    solver.assert(&(&z + &t1 * &vz - &t1 * v1_z)._eq(&p1_z));

    solver.assert(&(&x + &t2 * &vx - &t2 * v2_x)._eq(&p2_x));
    solver.assert(&(&y + &t2 * &vy - &t2 * v2_y)._eq(&p2_y));
    solver.assert(&(&z + &t2 * &vz - &t2 * v2_z)._eq(&p2_z));

    solver.assert(&(&x + &t3 * &vx - &t3 * v3_x)._eq(&p3_x));
    solver.assert(&(&y + &t3 * &vy - &t3 * v3_y)._eq(&p3_y));
    solver.assert(&(&z + &t3 * &vz - &t3 * v3_z)._eq(&p3_z));

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().expect("solver.get_model");
            println!("model = {model:?}");
            let answer = model.eval(&(x + y + z), true).expect("model.eval");
            Ok(answer.as_u64().unwrap() as usize)
        }
        _ => panic!("assertions are not satisfiable"),
    }
}
*/

#[rustfmt::skip]
#[allow(dead_code)]
fn part2_solve_mathru(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut hail = vec![];
    for s in &puzzle_lines[0..3] {
        let parse = trim_split_on::<f64>(&s.replace('@', ","), ',')?;
        hail.push((parse[0], parse[1], parse[2], parse[3], parse[4], parse[5]));
    }

    // points and velocities from the input
    let (p1_x, p1_y, p1_z, v1_x, v1_y, v1_z) = (
        hail[0].0, hail[0].1, hail[0].2, hail[0].3, hail[0].4, hail[0].5,
    );

    let (p2_x, p2_y, p2_z, v2_x, v2_y, v2_z) = (
        hail[1].0, hail[1].1, hail[1].2, hail[1].3, hail[1].4, hail[1].5,
    );

    let (p3_x, p3_y, p3_z, v3_x, v3_y, v3_z) = (
        hail[2].0, hail[2].1, hail[2].2, hail[2].3, hail[2].4, hail[2].5,
    );

    // coefficient matrix (note the transpose() when building from a vec![])
    let a: General<f64> = General::new(6, 6,
        vec![
            v2_y - v1_y , v1_x - v2_x , 0.            , p1_y - p2_y , p2_x - p1_x , 0.          ,
            v3_y - v1_y , v1_x - v3_x , 0.            , p1_y - p3_y , p3_x - p1_x , 0.          ,
            0.          , v2_z - v1_z , v1_y - v2_y , 0.            , p1_z - p2_z , p2_y - p1_y ,
            0.          , v3_z - v1_z , v1_y - v3_y , 0.            , p1_z - p3_z , p3_y - p1_y ,
            v2_z - v1_z , 0.          , v1_x - v2_x , p1_z - p2_z , 0.            , p2_x - p1_x ,
            v3_z - v1_z , 0.          , v1_x - v3_x , p1_z - p3_z , 0.            , p3_x - p1_x
        ]).transpose();

    // solution vector
    let b: Vector<f64> = Vector::new_column(
        vec![
            (p1_y * v1_x - p2_y * v2_x) - (p1_x * v1_y - p2_x * v2_y),
            (p1_y * v1_x - p3_y * v3_x) - (p1_x * v1_y - p3_x * v3_y),
            (p1_z * v1_y - p2_z * v2_y) - (p1_y * v1_z - p2_y * v2_z),
            (p1_z * v1_y - p3_z * v3_y) - (p1_y * v1_z - p3_y * v3_z),
            (p1_z * v1_x - p2_z * v2_x) - (p1_x * v1_z - p2_x * v2_z),
            (p1_z * v1_x - p3_z * v3_x) - (p1_x * v1_z - p3_x * v3_z)
        ]);

    // Solve it
    let solution: Vector<f64> = a.solve(&b).expect("mathru solve() failed");

    // no transpose() needed if using macros, matrix![], vector![]
    let solution2 = matrix![
            v2_y - v1_y , v1_x - v2_x , 0.          , p1_y - p2_y , p2_x - p1_x , 0.          ;
            v3_y - v1_y , v1_x - v3_x , 0.          , p1_y - p3_y , p3_x - p1_x , 0.          ;
            0.          , v2_z - v1_z , v1_y - v2_y , 0.          , p1_z - p2_z , p2_y - p1_y ;
            0.          , v3_z - v1_z , v1_y - v3_y , 0.          , p1_z - p3_z , p3_y - p1_y ;
            v2_z - v1_z , 0.          , v1_x - v2_x , p1_z - p2_z , 0.          , p2_x - p1_x ;
            v3_z - v1_z , 0.          , v1_x - v3_x , p1_z - p3_z , 0.          , p3_x - p1_x
        ].solve(&vector![
            (p1_y * v1_x - p2_y * v2_x) - (p1_x * v1_y - p2_x * v2_y);
            (p1_y * v1_x - p3_y * v3_x) - (p1_x * v1_y - p3_x * v3_y);
            (p1_z * v1_y - p2_z * v2_y) - (p1_y * v1_z - p2_y * v2_z);
            (p1_z * v1_y - p3_z * v3_y) - (p1_y * v1_z - p3_y * v3_z);
            (p1_z * v1_x - p2_z * v2_x) - (p1_x * v1_z - p2_x * v2_z);
            (p1_z * v1_x - p3_z * v3_x) - (p1_x * v1_z - p3_x * v3_z)
        ]).expect("mathru solve() failed");

    assert_eq!(solution, solution2);
    Ok((solution[0] + solution[1] + solution[2]).round() as usize)
}

#[rustfmt::skip]
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut hail = vec![];

    // any 3 points provide an integer solution (current algorithm)
    //
    // I originally looped over all stones taking 3 at a time, hashing
    // solver results and using the first duplicate answer to get
    // around floating point differences and that works fine too
    // for those class of solvers but this feels a little cleaner.
    // The z3 solver solution with time is nice.
    for s in &puzzle_lines[0..3] {
        let parse = trim_split_on::<i64>(&s.replace('@', ","), ',')?;
        hail.push((parse[0], parse[1], parse[2], parse[3], parse[4], parse[5]));
    }

    // points and velocities from the input
    let (p1_x, p1_y, p1_z, v1_x, v1_y, v1_z) = (
        hail[0].0, hail[0].1, hail[0].2, hail[0].3, hail[0].4, hail[0].5,
    );

    let (p2_x, p2_y, p2_z, v2_x, v2_y, v2_z) = (
        hail[1].0, hail[1].1, hail[1].2, hail[1].3, hail[1].4, hail[1].5,
    );

    let (p3_x, p3_y, p3_z, v3_x, v3_y, v3_z) = (
        hail[2].0, hail[2].1, hail[2].2, hail[2].3, hail[2].4, hail[2].5,
    );

    // coefficient matrix
    let a = [
        v2_y - v1_y , v1_x - v2_x , 0             , p1_y - p2_y , p2_x - p1_x , 0           ,
        v3_y - v1_y , v1_x - v3_x , 0             , p1_y - p3_y , p3_x - p1_x , 0           ,
        0           , v2_z - v1_z , v1_y - v2_y , 0             , p1_z - p2_z , p2_y - p1_y ,
        0           , v3_z - v1_z , v1_y - v3_y , 0             , p1_z - p3_z , p3_y - p1_y ,
        v2_z - v1_z , 0           , v1_x - v2_x , p1_z - p2_z , 0             , p2_x - p1_x ,
        v3_z - v1_z , 0           , v1_x - v3_x , p1_z - p3_z , 0             , p3_x - p1_x
    ];

    // solution vector
    let b = [
        (p1_y * v1_x - p2_y * v2_x) - (p1_x * v1_y - p2_x * v2_y),
        (p1_y * v1_x - p3_y * v3_x) - (p1_x * v1_y - p3_x * v3_y),
        (p1_z * v1_y - p2_z * v2_y) - (p1_y * v1_z - p2_y * v2_z),
        (p1_z * v1_y - p3_z * v3_y) - (p1_y * v1_z - p3_y * v3_z),
        (p1_z * v1_x - p2_z * v2_x) - (p1_x * v1_z - p2_x * v2_z),
        (p1_z * v1_x - p3_z * v3_x) - (p1_x * v1_z - p3_x * v3_z),
    ];
    
    // =============
    // Cramer's Rule
    // =============

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
    writeln!(stdout, "Answer Part 2 = {:?}", part2_solve_mathru(&puzzle_lines)?)?;
    //writeln!(stdout, "Answer Part 2 = {:?}", part2_z3_solver(&puzzle_lines)?)?;

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
    fn part2_solve_mathru_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2_solve_mathru(&puzzle_lines)?, 47);
        Ok(())
    }

    /*
    #[test]
    fn part2_z3_solver_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2_z3_solver(&puzzle_lines)?, 47);
        Ok(())
    }
    */

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 554668916217145);
        Ok(())
    }

    #[test]
    fn part2_solve_mathru_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2_solve_mathru(&puzzle_lines)?, 554668916217145);
        Ok(())
    }

    /*
    #[test]
    fn part2_z3_solver_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2_z3_solver(&puzzle_lines)?, 554668916217145);
        Ok(())
    }
    */
}
