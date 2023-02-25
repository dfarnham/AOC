use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use itertools::Itertools;
use num_integer::gcd;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

// I'm sure there are faster ways to implement this.
//
// I chose to solve these problems with lists of slopes between points.
//
// I'm keeping the slopes in rise,run format so all data types remain i64.
// This requires using (gcd, lcm) for comparisons and handling denominators of 0
//
// Lots of sorting but it works

fn get_data(puzzle_lines: &[String]) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let mut points = vec![];
    for (y, line) in puzzle_lines.iter().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            points.push((x as i64, y as i64));
        }
    }
    Ok(points)
}

fn get_slopes(points: &[(i64, i64)]) -> Vec<((i64, i64), i64, i64)> {
    // compute the pairwise slopes over the points
    //
    // collect a list of (point, rise, run).
    // where rise and run have been reduced by gcd
    //
    // combinations(2) generates the pairwise points
    let mut slopes = vec![];
    for point_pair in points.iter().combinations(2).collect::<Vec<_>>() {
        let (point1, point2) = (point_pair[0], point_pair[1]);

        let x_change = point1.0 - point2.0;
        let y_change = point1.1 - point2.1;
        let gcd = gcd(x_change, y_change);

        let (rise, run) = (y_change / gcd, x_change / gcd);

        // rise & run for each point to the other
        slopes.push((*point1, rise, run));
        slopes.push((*point2, -rise, -run));
    }
    slopes
}

fn get_counts(points: &[(i64, i64)]) -> Vec<((i64, i64), usize)> {
    let slopes = get_slopes(points);

    // collect the Vec of (point, rise, run) into a Set
    //
    // only count points in direct line of sight
    // (set collisions handle this)
    let slopeset = slopes.iter().collect::<HashSet<_>>();
    points
        .iter()
        .map(|p| (*p, slopeset.iter().filter(|(point, _, _)| point == p).count()))
        .collect()
}

#[rustfmt::skip]
fn clockwise(point: (i64, i64), points: &[((i64, i64), i64)]) -> Vec<((i64, i64), i64)> {
    let mut quad1 = vec![];
    let mut quad2 = vec![];
    let mut quad3 = vec![];
    let mut quad4 = vec![];

    // put the points into quadrants relative to the input point
    for p in points.iter().filter(|p| p.0 != point) {
        match p.0 {
            (x, y) if x >= point.0 && y <= point.1 => { quad1.push(*p); }
            (x, y) if x >= point.0 && y >= point.1 => { quad2.push(*p); }
            (x, y) if x <= point.0 && y >= point.1 => { quad3.push(*p); }
            _ => { quad4.push(*p); }
        }
    }

    // Radar Sweep Order
    // -----------------
    //
    // sort the points in quadrant 1:
    //  slope (high to low), then sub sort by x coord (low to high), then y coord (high to low)
    quad1.sort_by(|((x1, y1), s1), ((x2, y2), s2)| s2.cmp(s1).then(x1.cmp(x2)).then(y2.cmp(y1)));

    // sort the points in quadrant 2:
    //  slope (low to high), then sub sort by x coord (low to high), then y coord (low to high)
    quad2.sort_by(|((x1, y1), s1), ((x2, y2), s2)| s1.cmp(s2).then(x1.cmp(x2)).then(y1.cmp(y2)));

    // sort the points in quadrant 3:
    //  slope (high to low), then sub sort by x coord (high to low), then y coord (low to high)
    quad3.sort_by(|((x1, y1), s1), ((x2, y2), s2)| s2.cmp(s1).then(x2.cmp(x1)).then(y1.cmp(y2)));

    // sort the points in quadrant 4:
    //  slope (low to high), then sub sort by x coord (high to low), then y coord (high to low)
    quad4.sort_by(|((x1, y1), s1), ((x2, y2), s2)| s1.cmp(s2).then(x2.cmp(x1)).then(y2.cmp(y1)));

    // append the "quadrant sorted" points for 2,3,4 to quad1
    quad1.extend(quad2);
    quad1.extend(quad3);
    quad1.extend(quad4);

    quad1
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let points = get_data(puzzle_lines)?;
    Ok(get_counts(&points).iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let points = get_data(puzzle_lines)?;

    let best_location = get_counts(&points).iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;

    // get_counts() already did a form of this, some refactoring to
    // of the part1(), part2() components would make this unnecessary
    //
    // gathering absolute rise, run since the comparisons are being
    // done in 4 separate quadrants, we just need a number to order on
    let mut slopes = vec![];
    for point in points.iter().filter(|p| **p != best_location) {
        let x_change = best_location.0 - point.0;
        let y_change = best_location.1 - point.1;
        let gcd = gcd(x_change, y_change);

        let (rise, run) = (y_change / gcd, x_change / gcd);

        slopes.push((*point, rise.abs(), run.abs()));
    }

    // make all the rise, run pairs comparable
    let denominators = slopes.iter().map(|tup| tup.2).collect::<Vec<_>>();
    let lcm = denominators
        .iter()
        .filter(|x| *x != &0)
        .fold(denominators[0], |lcm, x| num_integer::lcm(lcm, *x));

    // this is a little wonky, i64::MAX is a surragate for inf (infinite slope)
    let mut norm_points = vec![];
    for (p, rise, run) in slopes {
        if run == 0 {
            norm_points.push((p, i64::MAX));
        } else {
            norm_points.push((p, rise * (lcm / run)));
        }
    }

    // clockwise() returns a list, a concatenation of quadrants 1-4
    // where each quadrant has been "sweep" ordered
    //
    // use a work queue to keep cycling through the points
    let mut list = clockwise(best_location, &norm_points)
        .iter()
        .copied()
        .collect::<VecDeque<_>>();

    // asteroids are: ((x, y), slope)
    let mut last = (best_location, -1);
    let mut count = 0;
    while let Some(asteroid) = list.pop_front() {
        if asteroid.1 == last.1 {
            if list.is_empty() {
                break;
            }
            // same slope as the last point, go to the back of the list for the next sweep
            list.push_back(asteroid);
            continue;
        }

        count += 1;
        if count == 200 {
            return Ok(asteroid.0 .0 * 100 + asteroid.0 .1);
        }

        last = asteroid;
    }

    Ok(-1)
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines)?)?;

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
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 8);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 340);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, -1);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2628);
        Ok(())
    }
}