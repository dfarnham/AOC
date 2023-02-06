use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_target_area(data: &str) -> (i64, i64, i64, i64) {
    let re = Regex::new(r"target\s+area:\s+x=(\d+)\.\.(\d+),\s+y=(-\d+)\.\.(-\d+)").unwrap();
    let captures = re.captures(data).unwrap();
    (
        captures.get(1).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(2).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(3).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(4).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
    )
}

fn solutions(data: &str) -> (i64, usize) {
    let (xmin, xmax, ymin, ymax) = get_target_area(data);

    let mut best_y = i64::MIN;
    let mut velocity = HashSet::new();

    for n in ((2.0 * xmin as f64).sqrt().round() as i64)..=xmax {
        for m in ymin..ymin.abs() {
            let (mut x, mut y) = (0, 0);
            let (mut xv, mut yv) = (n, m);
            let mut max_y = 0;
            let mut success = false;
            for _step in 0..(2 * ymin.abs()) {
                if x + xv == x && (x < xmin || x > xmax) {
                    break;
                }

                x += xv;
                y += yv;
                xv = match xv < 0 {
                    true => xv + 1,
                    false => 0.max(xv - 1),
                };
                yv -= 1;

                if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                    success = true;
                    velocity.insert((n, m));
                }
                max_y = max_y.max(y);
            }
            if success && max_y > best_y {
                best_y = max_y;
            }
        }
    }
    (best_y, velocity.len())
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

    let (best, n) = solutions(&puzzle_lines[0]);
    writeln!(stdout, "Answer Part 1 = {best:?}")?;
    writeln!(stdout, "Answer Part 2 = {n:?}")?;

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
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(solutions(&data[0]).0, 45);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(solutions(&data[0]).0, 6786);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(solutions(&data[0]).1, 112);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(solutions(&data[0]).1, 2313);
    }
}
