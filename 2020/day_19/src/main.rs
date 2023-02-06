use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use regex::Regex;
use std::collections::HashMap;

fn build_re(key: &str, rules: &HashMap<String, Vec<Vec<String>>>) -> String {
    let mut s1 = String::new();
    let mut s2 = String::new();

    match rules.get(key) {
        Some(v) => {
            for item in &v[0] {
                s1.push_str(&build_re(item, rules));
            }

            // v.len() == 2 if this is an alternation:  part1 | part2
            if v.len() > 1 {
                // check for a self-reference (cycle)
                if v[1].contains(&key.to_string()) {
                    if v[1].len() == 2 {
                        // example: rule 8 can be simplified to 42+
                        //   "8: 42 | 42 8"
                        s1.push('+');
                    } else {
                        // example: rule 11 turns into 42{n}, (42 31), 31{n}
                        //   "11: 42 31 | 42 11 31"
                        let before = build_re(&v[1][0], rules);
                        let after = build_re(&v[1][2], rules);

                        // hedge that 9 symmetrical before/after surrounding v[0] will suffice
                        for i in 1..9 {
                            s2.push_str(&format!("|{before}{{{i}}}{s1}{after}{{{i}}}"));
                        }
                    }
                } else {
                    s2.push('|');
                    for item in &v[1] {
                        s2.push_str(&build_re(item, rules));
                    }
                }
            }
            "(".to_owned() + &s1 + &s2 + ")"
        }
        None => key.to_string(),
    }
}

fn solution1(data: &[String]) -> usize {
    let mut rules = HashMap::new();

    for (count, line) in data.iter().enumerate() {
        if line.is_empty() {
            let re = Regex::new(&("^".to_owned() + &build_re("0", &rules) + "$")).unwrap();
            return data[count + 1..].iter().filter(|s| re.is_match(s)).count();
        } else {
            let mut groups = vec![];
            let parts = trim_split_on::<String>(&line.replace('"', ""), ':').unwrap();
            for group in trim_split_on::<String>(&parts[1], '|').unwrap() {
                let items = group.split_whitespace().map(|s| s.into()).collect();
                groups.push(items);
            }
            rules.insert(parts[0].to_string(), groups);
        }
    }
    panic!("missing empty line")
}

fn solution2(data: &[String]) -> usize {
    let mut cycle_data = vec![];
    // replace rules 8 and 11 with rules which cycle
    for line in data {
        if line == "8: 42" {
            cycle_data.push("8: 42 | 42 8".into());
        } else if line == "11: 42 31" {
            cycle_data.push("11: 42 31 | 42 11 31".into());
        } else {
            cycle_data.push(line.into());
        }
    }
    solution1(&cycle_data)
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

    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&puzzle_lines))?;

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
        assert_eq!(2, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(156, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(2, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(363, solution2(&data));
    }
}
