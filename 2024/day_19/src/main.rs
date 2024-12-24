use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use pathfinding::prelude::count_paths;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let patterns = trim_split_on::<String>(&data[0], ',')?;
    let designs = &data[1..]
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    Ok((patterns, designs.to_vec()))
}

#[allow(dead_code)]
// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2024/19.py
fn solve_better(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let (patterns, designs) = get_data(puzzle_lines)?;

    fn ways(patterns: &[String], design: &str, visited: &mut HashMap<String, usize>) -> usize {
        if visited.contains_key(design) {
            return visited[design];
        }
        let mut count = 0;
        if design.is_empty() {
            count = 1;
        }
        for pattern in patterns {
            if design.starts_with(pattern) {
                count += ways(patterns, &design[pattern.len()..], visited);
            }
        }
        visited.insert(design.to_string(), count);
        count
    }

    Ok(match part2 {
        false => designs
            .iter()
            .filter(|design| ways(&patterns, design, &mut HashMap::new()) > 0)
            .count(),
        true => designs
            .iter()
            .map(|design| ways(&patterns, design, &mut HashMap::new()))
            .sum(),
    })
}

fn solve(puzzle_lines: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let (patterns, designs) = get_data(puzzle_lines)?;

    // shortest path priority-q to find a valid solution
    let is_valid = |design: &str| -> bool {
        let mut visited = HashSet::new();
        let mut workq = VecDeque::new();
        workq.push_back(design);
        while let Some(s) = workq.pop_front() {
            if !visited.contains(&s) {
                visited.insert(s);

                if s.is_empty() {
                    return true;
                }
                for pat in patterns.iter().filter(|pat| s.starts_with(*pat)) {
                    workq.push_back(&s[pat.len()..]);
                }
            }
        }
        false
    };

    Ok(match part2 {
        false => designs.iter().filter(|design| is_valid(design)).count(),
        true => designs
            .iter()
            // filter on valid designs ( not required )
            .filter(|design| is_valid(design))
            .map(|design| {
                // build a graph from the design; a map of indices => sets of indices
                //
                // keys are the design indicies 0..design.len()
                // values are the set of indices reachable by a pattern match at each index
                let mut g = HashMap::new();
                for i in 0..design.len() {
                    let indices: HashSet<_> = patterns
                        .iter()
                        .filter(|pattern| design[i..].starts_with(*pattern))
                        .map(|pattern| i + pattern.len())
                        .collect();
                    g.insert(i, indices);
                }
                // count the paths for this graph
                count_paths(
                    // starting index
                    0,
                    // set of reachable indices from index
                    |index| g[index].clone(),
                    // stopping condition
                    |index| *index == design.len(),
                )
            })
            .sum(),
    })
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
        assert_eq!(solve(&puzzle_lines, false)?, 6);
        assert_eq!(solve_better(&puzzle_lines, false)?, 6);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 315);
        assert_eq!(solve_better(&puzzle_lines, false)?, 315);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 16);
        assert_eq!(solve_better(&puzzle_lines, true)?, 16);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 625108891232249);
        assert_eq!(solve_better(&puzzle_lines, true)?, 625108891232249);
        Ok(())
    }
}
