use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn solve(data: &[String], part2: bool) -> Result<usize, Box<dyn Error>> {
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut loading_rules = true;
    let mut midpoint_total = 0;

    for line in data {
        if line.is_empty() {
            loading_rules = false;
            continue;
        }

        if loading_rules {
            // page ordering rules, one per line
            //
            // 47|53
            // 97|13
            // 97|61
            //  ...
            //
            //  if an update includes both page number 47 and page number 53,
            //  then page number 47 must be printed at some point before page number 53.
            for rule in trim_split_on::<usize>(line, '|')?.chunks(2) {
                let (key, value) = (rule[0], rule[1]);
                let set = rules.entry(key).or_default();
                set.insert(value);
            }
        } else {
            // page ordering updates, one per line (may or may not be a valid ordering)
            //
            // 75,47,61,53,29
            // 97,61,53,29,13
            // 75,29,13
            let mut updates = VecDeque::from(trim_split_on::<usize>(line, ',')?);
            let mut fixed_updates = VecDeque::new();
            let mut valid = true;
            while let Some(n) = updates.pop_back() {
                if let Some(rule_set) = rules.get(&n) {
                    if rule_set.is_disjoint(&HashSet::from_iter(updates.clone())) {
                        fixed_updates.push_front(n);
                    } else {
                        valid = false;
                        if !part2 {
                            break;
                        }
                        updates.push_front(n);
                    }
                }
            }

            // puzzle answer is a sum of midpoint values
            if (!part2 && valid) || (part2 && !valid) {
                midpoint_total += fixed_updates[fixed_updates.len() / 2];
            }
        }
    }
    Ok(midpoint_total)
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
        assert_eq!(solve(&puzzle_lines, false)?, 143);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, false)?, 5588);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(solve(&puzzle_lines, true)?, 123);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(solve(&puzzle_lines, true)?, 5331);
        Ok(())
    }
}
