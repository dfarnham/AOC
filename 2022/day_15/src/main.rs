use general::{get_args, read_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};
use std::ops::RangeInclusive;

fn get_data(data: &[String]) -> HashMap<(i64, i64), (i64, i64, i64)> {
    // ex.                Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let sensor_re = Regex::new(r".*?x=(\d+).*?y=(\d+).*?x=([-]?\d+).*?y=([-]?\d+)").unwrap();
    let mut sensors = HashMap::new();
    for line in data {
        if sensor_re.is_match(line) {
            let captures = sensor_re.captures(line).unwrap();
            let points = (1..=4)
                .map(|n| {
                    captures
                        .get(n)
                        .map(|s| s.as_str().parse::<i64>().unwrap())
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let d = (points[0] - points[2]).abs() + (points[1] - points[3]).abs();
            sensors.insert((points[0], points[1]), (points[2], points[3], d));
        }
    }
    sensors
}

// merges ranges
// ex:
//   before [15..=17, 15..=20, 0..=3, 3..=13, 11..=13]
//   after [15..=20, 0..=13]
fn merge_ranges(r: &mut Vec<RangeInclusive<i64>>) {
    // normalize ranges into low..=high
    for item in &mut r.iter_mut() {
        if item.start() > item.end() {
            *item = *item.end()..=*item.start();
        }
    }

    // merge overlapping ranges
    let mut done = false;
    while !done {
        done = true;
        'outer: for i in 0..r.len() {
            let (start_i, end_i) = (r[i].start(), r[i].end());
            for j in 0..r.len() {
                if i == j {
                    continue;
                } else if r[j].start() >= start_i && r[j].start() <= end_i {
                    if r[j].end() > end_i {
                        r[i] = *start_i..=*r[j].end();
                    }
                    r.remove(j);
                    done = false;
                    break 'outer;
                } else if *r[j].end() + 1 == *start_i {
                    r[i] = *r[j].start()..=*end_i;
                    r.remove(j);
                    done = false;
                    break 'outer;
                }
            }
        }
    }
}

// Part1 solving at the expense of space because I'll be generating
// all the points that intersect a line for each sensor and collecting
// them into a set.  The size of the set is the final answer
//
// two points (x1, y1), (x2, y2) which represent a sensor and its beacon.
//
// sensors is a HashMap keyed on (x1, y1) with value (x2, y2, d) where d
// is the Manhatten distance
#[allow(dead_code)]
fn solve1_not_scaleable(puzzle_lines: &[String], row: usize) -> Result<usize, Box<dyn Error>> {
    let row = row as i64;

    let sensors = get_data(puzzle_lines);

    let mut s = HashSet::new();
    for (k, v) in sensors.iter() {
        let (x1, y1, x2, y2, d) = (k.0, k.1, v.0, v.1, v.2);

        if (d - y1).abs() > row {
            continue;
        }

        let mag = d - (row - y1).abs();
        if mag > 0 {
            // generate the points in ranges x1 += mag skipping any beacons on the line
            for i in x1 - mag..=x1 + mag {
                // don't count the beacon if it's in the input row
                if y2 != row || i != x2 {
                    s.insert(i);
                }
            }
        }
    }
    Ok(s.len())
}

// two points (x1, y1), (x2, y2) which represent a sensor and its beacon.
//
// sensors is a HashMap keyed on (x1, y1) with value (x2, y2, d) where d
// is the Manhatten distance
//
fn solve1(puzzle_lines: &[String], row: usize) -> Result<usize, Box<dyn Error>> {
    let row = row as i64;

    let sensors = get_data(puzzle_lines);

    let mut row_coverage = vec![];
    for (k, v) in sensors.iter() {
        let (x1, y1, _x2, _y2, d) = (k.0, k.1, v.0, v.1, v.2);

        // collect the coverage ranges on the row (x1 += mag)
        if row >= y1 - d && row <= y1 + d {
            let mag = d - (row - y1).abs();
            if mag > 0 {
                row_coverage.push(x1 - mag..=x1 + mag);
            }
        }
    }

    // beacons on the row
    let beacons = sensors
        .values()
        .filter(|(_, y, _)| *y == row)
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<(_, _)>>();

    // add the row_coverage range distances, don't count beacons
    merge_ranges(&mut row_coverage);
    Ok(row_coverage
        .iter()
        .map(|r| *r.end() - *r.start() + 1)
        .sum::<i64>() as usize
        - beacons.len())
}

fn solve2(puzzle_lines: &[String], maxp: usize) -> Result<usize, Box<dyn Error>> {
    let sensors = get_data(puzzle_lines);
    let maxpi = maxp as i64;

    let mut x = None;
    let mut y = None;

    for i in (0..maxpi).rev() {
        let mut row_coverage = vec![];
        let mut col_coverage = vec![];

        for (k, v) in sensors.iter() {
            let (x1, y1, _x2, _y2, d) = (k.0, k.1, v.0, v.1, v.2);

            // collect the coverage ranges on the row (x1 += mag)
            if y.is_none() && i >= y1 - d && i <= y1 + d {
                let mag = d - (i - y1).abs();
                if mag > 0 {
                    row_coverage.push(0.max(x1 - mag)..=maxpi.min(x1 + mag));
                }
            }

            // collect the coverage ranges on the col (y1 += mag)
            if x.is_none() && i >= x1 - d && i <= x1 + d {
                let mag = d - (i - x1).abs();
                if mag > 0 {
                    col_coverage.push(0.max(y1 - mag)..=maxpi.min(y1 + mag));
                }
            }
        }

        if y.is_none() {
            merge_ranges(&mut row_coverage);
            if row_coverage.len() > 1 {
                y = Some(i as usize);
            }
        }

        if x.is_none() {
            merge_ranges(&mut col_coverage);
            if col_coverage.len() > 1 {
                x = Some(i as usize);
            }
        }

        if let (Some(x), Some(y)) = (x, y) {
            return Ok(x * 4000000 + y);
        }
    }

    panic!("no solution")
}

fn part1(puzzle_lines: &[String], row: usize) -> Result<usize, Box<dyn Error>> {
    solve1(puzzle_lines, row)
}

fn part2(puzzle_lines: &[String], maxrow: usize) -> Result<usize, Box<dyn Error>> {
    solve2(puzzle_lines, maxrow)
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

    // guess input is input-example or input-actual by length
    let arg1 = match puzzle_lines.len() {
        n if n == 24 => 2000000,
        _ => 10,
    };
    let arg2 = match puzzle_lines.len() {
        n if n == 24 => 4000000,
        _ => 20,
    };
    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines, arg1)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines, arg2)?)?;

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
        assert_eq!(part1(&puzzle_lines, 10)?, 26);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines, 2000000)?, 5040643);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines, 20)?, 56000011);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines, 4000000)?, 11016575214126);
        Ok(())
    }
}
