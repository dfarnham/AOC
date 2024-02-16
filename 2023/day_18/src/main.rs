use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use geo::*;
use pathfinding::matrix::*;
use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

#[allow(dead_code)]
fn display(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut points = BTreeSet::new();
    let mut cur = (0_i32, 0_i32);
    points.insert(cur);
    let mut line_string = vec![];
    line_string.push((cur.0 as f64, cur.1 as f64));
    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;
        let length = parts[1].parse::<usize>()?;
        let _rgb = &parts[2][2..parts[2].len() - 1].to_string();
        match parts[0].as_ref() {
            "U" => {
                for _ in 0..length {
                    cur = (cur.0 - 1, cur.1);
                    points.insert(cur);
                }
                line_string.push((cur.0 as f64, cur.1 as f64));
            }
            "D" => {
                for _ in 0..length {
                    cur = (cur.0 + 1, cur.1);
                    points.insert(cur);
                }
                line_string.push((cur.0 as f64, cur.1 as f64));
            }
            "L" => {
                for _ in 0..length {
                    cur = (cur.0, cur.1 - 1);
                    points.insert(cur);
                }
                line_string.push((cur.0 as f64, cur.1 as f64));
            }
            "R" => {
                for _ in 0..length {
                    cur = (cur.0, cur.1 + 1);
                    points.insert(cur);
                }
                line_string.push((cur.0 as f64, cur.1 as f64));
            }
            _ => unreachable!(),
        }
    }
    //println!("line_string = {line_string:?}");
    let minr = points.iter().map(|k| k.0).min().unwrap();
    let maxr = points.iter().map(|k| k.0).max().unwrap();
    let minc = points.iter().map(|k| k.1).min().unwrap();
    let maxc = points.iter().map(|k| k.1).max().unwrap();
    //println!("{minr:?}, {maxr:?}, {minc:?}, {maxc:?}");
    let mut m = Matrix::new(1 + (maxr - minr) as usize, 1 + (maxc - minc) as usize, '.');
    for k in points {
        let (i, j) = ((k.0 - minr) as usize, (k.1 - minc) as usize);
        m[(i, j)] = '#';
    }

    let mut p = (0, 0);
    'outer: for i in 0..m.rows {
        for j in 0..m.columns {
            if m[(i, j)] == '#' {
                p = (i + 1, j + 1);
                break 'outer;
            }
        }
    }
    let mut workq = VecDeque::new();
    workq.push_back(p);
    while let Some(p) = workq.pop_front() {
        if m[p] == '#' {
            continue;
        }
        for n in m.neighbours(p, false).filter(|p| m[*p] != '#') {
            workq.push_back(n);
        }
        m[p] = '#';
    }
    let mut total = 0;
    for i in 0..m.rows {
        for j in 0..m.columns {
            //print!("{}", m[(i, j)]);
            if m[(i, j)] == '#' {
                total += 1
            }
        }
        //println!();
    }
    Ok(total)
}

// Input: a list of coordinates representing a connected point path
// Output: area and perimiter length
// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace(line_string: &[(i64, i64)]) -> (usize, usize) {
    let manhatten = |a: (i64, i64), b: (i64, i64)| -> usize { ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize };

    // walk the point path by pairs
    let (shoelace, perimiter) = line_string.windows(2).fold((0, 0), |(shoe, perim), p| {
        (
            // shoelace
            shoe + p[0].0 * p[1].1 - p[0].1 * p[1].0,
            // perimiter
            perim + manhatten(p[0], p[1]),
        )
    });

    ((shoelace / 2).unsigned_abs() as usize, perimiter)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut line_string = vec![];
    let mut cur = (0_i64, 0_i64);
    let mut perim = 0;
    line_string.push((0, 0));

    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;
        let length = parts[1].parse::<usize>()?;
        let _rgb = &parts[2][2..parts[2].len() - 1].to_string();
        let d = length as i64;
        perim += d;
        match parts[0].as_ref() {
            "U" => {
                cur = (cur.0 - d, cur.1);
                line_string.push(cur);
            }
            "D" => {
                cur = (cur.0 + d, cur.1);
                line_string.push(cur);
            }
            "L" => {
                cur = (cur.0, cur.1 - d);
                line_string.push(cur);
            }
            "R" => {
                cur = (cur.0, cur.1 + d);
                line_string.push(cur);
            }
            _ => unreachable!(),
        }
    }

    let (shoelace, tperim) = shoelace(&line_string);

    // could have used geo::*
    let ls: LineString<f64> = line_string.iter().map(|(r, c)| (*r as f64, *c as f64)).collect();
    let poly = Polygon::new(ls, vec![]);
    assert_eq!(poly.unsigned_area() as usize, shoelace);

    // we calculated the perimiter 2 ways
    assert_eq!(perim, tperim as i64);

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    Ok(1 + shoelace + (perim / 2) as usize)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut line_string = vec![];
    let mut cur = (0_i64, 0_i64);
    let mut perim = 0;
    line_string.push((0, 0));

    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;
        let _length = parts[1].parse::<usize>()?;
        let rgb = &parts[2][2..parts[2].len() - 1].to_string();
        let d = i64::from_str_radix(&rgb[0..5], 16)?;
        perim += d;
        match rgb[5..6].as_ref() {
            "3" => {
                cur = (cur.0 - d, cur.1);
                line_string.push(cur);
            }
            "1" => {
                cur = (cur.0 + d, cur.1);
                line_string.push(cur);
            }
            "2" => {
                cur = (cur.0, cur.1 - d);
                line_string.push(cur);
            }
            "0" => {
                cur = (cur.0, cur.1 + d);
                line_string.push(cur);
            }
            _ => unreachable!(),
        }
    }
    let (shoelace, tperim) = shoelace(&line_string);
    assert_eq!(perim, tperim as i64);
    Ok(1 + shoelace + (perim / 2) as usize)
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
        assert_eq!(part1(&puzzle_lines)?, 62);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 39039);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 952408144115);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 44644464596918);
        Ok(())
    }
}
