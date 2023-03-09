use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (String, HashMap<String, String>) {
    let mut productions = HashMap::new();
    for line in data[1..].iter().filter(|s| !s.is_empty()) {
        let pairs = line.split("->").map(|s| s.trim().to_string()).collect::<Vec<_>>();
        productions.insert(pairs[0].to_string(), pairs[1].to_string());
    }
    (data[0].to_string(), productions)
}

fn offset_slices(s: &str, n: usize) -> Vec<&str> {
    (0..s.len() - n + 1).map(|i| &s[i..i + n]).collect()
}

fn pair_counts(
    pair: &str,
    productions: &HashMap<String, String>,
    cntr: &mut Counter<String, usize>,
    cache: &mut HashMap<String, Counter<String, usize>>,
    step: usize,
) {
    match productions.get(pair) {
        Some(rule) => {
            cntr[rule] += 1;
            if step > 1 {
                let left_pair = pair[0..1].to_string() + rule;
                let right_pair = rule.to_owned() + &pair[1..2];
                let key = format!("{step}:{left_pair}:{right_pair}");
                if let Some(saved_cntr) = cache.get(&key) {
                    for (k, v) in saved_cntr {
                        cntr[k] += *v;
                    }
                } else {
                    let saved_cntr = cntr.clone();
                    pair_counts(&left_pair, productions, cntr, cache, step - 1);
                    pair_counts(&right_pair, productions, cntr, cache, step - 1);
                    cache.insert(key, cntr.clone() - saved_cntr);
                }
            }
        }
        None => panic!("Unknown pair = {pair}"),
    }
}

fn solution(start: &str, productions: &HashMap<String, String>, steps: usize) -> usize {
    // initialize counter with starting chars
    let mut cntr = start.chars().map(|c| c.to_string()).collect::<Counter<_>>();

    // initialize a "pair => Counter" memoize cache
    let mut cache = HashMap::new();

    for pair in offset_slices(start, 2) {
        pair_counts(pair, productions, &mut cntr, &mut cache, steps);
    }

    // sort and return: most frequent - least frequent
    let by_common = cntr.most_common_ordered();
    by_common.first().unwrap().1 - by_common.last().unwrap().1
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

    let (start, productions) = get_data(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {:?}", solution(&start, &productions, 10))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution(&start, &productions, 40))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_testdata(filename: &str) -> (String, HashMap<String, String>) {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        let (start, productions) = get_data(&data);
        (start, productions)
    }

    #[test]
    fn part1_example() {
        let (start, productions) = get_testdata("input-example");
        assert_eq!(solution(&start, &productions, 10), 1588)
    }

    #[test]
    fn part1_actual() {
        let (start, productions) = get_testdata("input-actual");
        assert_eq!(solution(&start, &productions, 10), 2937)
    }

    #[test]
    fn part2_example() {
        let (start, productions) = get_testdata("input-example");
        assert_eq!(solution(&start, &productions, 40), 2188189693529)
    }

    #[test]
    fn part2_actual() {
        let (start, productions) = get_testdata("input-actual");
        assert_eq!(solution(&start, &productions, 40), 3390034818249)
    }
}
