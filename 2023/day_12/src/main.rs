use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on, trim_split_ws};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
fn f(
    dp: &mut HashMap<(usize, usize, usize), usize>,
    pattern: &[char],
    items: &[usize],
    i: usize,
    bi: usize,
    current: usize,
) -> usize {
    let key = (i, bi, current);
    if dp.contains_key(&key) {
        return dp[&key];
    }
    if i == pattern.len() {
        if bi == items.len() && current == 0 || bi == items.len() - 1 && items[bi] == current {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ans = 0;
    for c in ['.', '#'] {
        if pattern[i] == c || pattern[i] == '?' {
            if c == '.' && current == 0 {
                ans += f(dp, pattern, items, i + 1, bi, 0);
            } else if c == '.' && current > 0 && bi < items.len() && items[bi] == current {
                ans += f(dp, pattern, items, i + 1, bi + 1, 0);
            } else if c == '#' {
                ans += f(dp, pattern, items, i + 1, bi, current + 1);
            }
        }
    }
    dp.insert(key, ans);
    ans
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;
        let pattern: Vec<_> = parts[0].chars().collect();
        let items: Vec<_> = trim_split_on::<usize>(&parts[1], ',')?;
        ans += f(&mut HashMap::new(), &pattern, &items, 0, 0, 0);
    }

    Ok(ans)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;

        let mut p0 = parts[0].to_string();
        let mut p1 = parts[1].to_string();
        for _ in 0..4 {
            p0 += &("?".to_owned() + &parts[0]);
            p1 += &(",".to_owned() + &parts[1]);
        }

        let pattern: Vec<_> = p0.chars().collect();
        let items: Vec<_> = trim_split_on::<usize>(&p1, ',')?;
        ans += f(&mut HashMap::new(), &pattern, &items, 0, 0, 0);
    }

    Ok(ans)
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
        assert_eq!(part1(&puzzle_lines)?, 21);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 7032);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 525152);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 1493340882140);
        Ok(())
    }
}
