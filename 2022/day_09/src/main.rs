use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_pos(t: (i32, i32), m: (i32, i32)) -> (i32, i32) {
    match ((t.0 - m.0).abs(), (t.1 - m.1).abs()) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => t,
        (r, c) if r > 1 && c > 1 => (
            if m.0 > t.0 { t.0 + 1 } else { t.0 - 1 },
            if m.1 > t.1 { t.1 + 1 } else { t.1 - 1 },
        ),
        (r, _) if r > 1 => (if m.0 > t.0 { t.0 + 1 } else { t.0 - 1 }, m.1),
        _ => (m.0, if m.1 > t.1 { t.1 + 1 } else { t.1 - 1 }),
    }
}

fn move_head(v: &[(i32, i32)], m: (i32, i32)) -> Vec<(i32, i32)> {
    let mut newvec = vec![m];
    for t in v[1..].iter() {
        newvec.push(get_pos(*t, *newvec.last().unwrap()))
    }
    newvec
}

fn coverage(puzzle_lines: &[String], n: usize) -> Result<usize, Box<dyn Error>> {
    let mut mat = HashSet::new();
    let mut knots = (0..n).map(|_| (0, 0)).collect::<Vec<(_, _)>>();

    mat.insert(knots[n - 1]);
    for line in puzzle_lines {
        let mut cmd = line.split_whitespace();
        if let (Some(direction), Some(distance)) = (cmd.next(), cmd.next()) {
            let (row, col) = match direction {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (1, 0),
                "D" => (-1, 0),
                _ => return Err(Box::from(format!("Unknown direction: {direction}"))),
            };
            for _ in 0..distance.parse::<usize>()? {
                knots = move_head(&knots, (knots[0].0 + row, knots[0].1 + col));
                mat.insert(knots[n - 1]);
            }
        }
    }
    Ok(mat.len())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    coverage(puzzle_lines, 2)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    coverage(puzzle_lines, 10)
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
        assert_eq!(part1(&puzzle_lines)?, 13);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 6357);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 1);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part2(&puzzle_lines)?, 36);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2627);
        Ok(())
    }
}
