use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn solution(puzzle_lines: &[String], p1: bool) -> Result<usize, Box<dyn Error>> {
    let mut directions: VecDeque<_> = puzzle_lines[0].chars().collect();
    let mut nodes = HashMap::new();
    for line in puzzle_lines.iter().skip(2) {
        let line = line.replace(['=', '(', ',', ')'], "").to_string();
        let parts: Vec<_> = trim_split_ws::<String>(&line)?;
        nodes.insert(parts[0].to_string(), (parts[1].to_string(), parts[2].to_string()));
    }

    if p1 {
        let (start, end) = ("AAA", "ZZZ");
        let mut workq = VecDeque::from([(directions.pop_front(), &nodes[start])]);
        let mut steps = 0;

        while let Some((d, choices)) = workq.pop_front() {
            steps += 1;

            let (left, right) = choices;
            if d == Some('L') && left == end || d == Some('R') && right == end {
                break;
            }

            if d == Some('L') {
                workq.push_back((directions.pop_front(), &nodes[left]));
            } else {
                workq.push_back((directions.pop_front(), &nodes[right]));
            }
            directions.push_back(d.unwrap());
        }

        Ok(steps)
    } else {
        let orig_directions = directions.clone();
        let a_end: Vec<_> = nodes.keys().filter(|k| k.ends_with('A')).collect();

        let mut steps_z = HashMap::new();
        for item in a_end.into_iter() {
            let mut dir = orig_directions.clone();
            let mut key = item.clone();
            while let Some(d) = dir.pop_front() {
                *steps_z.entry(item).or_insert(0) += 1;
                key = match d == 'L' {
                    true => nodes[&key].0.clone(),
                    false => nodes[&key].1.clone(),
                };
                if key.ends_with('Z') {
                    break;
                }
                dir.push_back(d);
            }
        }
        let lcm = steps_z.values().copied().reduce(num_integer::lcm).expect("lcm error");
        Ok(lcm)
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, true)
}
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, false)
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
        assert_eq!(part1(&puzzle_lines)?, 2);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part1(&puzzle_lines)?, 6);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 15517);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 2);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3")?;
        assert_eq!(part2(&puzzle_lines)?, 6);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 14935034899483);
        Ok(())
    }
}
