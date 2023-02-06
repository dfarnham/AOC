use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn _count_containers(
    containers: &HashMap<String, HashSet<String>>,
    bags: &mut HashSet<String>,
    query_bag: &str,
) -> usize {
    if let Some(bagset) = containers.get(query_bag) {
        for bag in bagset {
            if !bags.contains(bag) {
                bags.insert(bag.clone());
                _count_containers(containers, bags, bag);
            }
        }
    }
    bags.len()
}

fn count_containers(containers: &HashMap<String, HashSet<String>>, query_bag: &str) -> usize {
    _count_containers(containers, &mut HashSet::<String>::new(), query_bag)
}

fn count_bags(bags: &HashMap<String, Vec<(String, usize)>>, query_bag: &str) -> usize {
    match bags.get(query_bag) {
        Some(contents) => 1 + contents.iter().map(|b| b.1 * count_bags(bags, &b.0)).sum::<usize>(),
        None => 0,
    }
}

fn solution1(data: &[String], query_bag: &str) -> usize {
    let re1: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)").unwrap();
    let re2: Regex = Regex::new(r"^\s*\d+ (\w+ \w+) bag").unwrap();

    // a map, keyed by bag-name, identifying the set of bags-names which contain the lookup
    let mut containers = HashMap::<String, HashSet<String>>::new();

    for line in data {
        let caps = re1.captures(line).unwrap();
        let id = caps.get(1).expect("capture failed").as_str().to_string();
        let rest = caps.get(2).expect("capture failed").as_str();

        for s in trim_split_on::<String>(rest, ',').unwrap() {
            if !s.contains("no other bag") {
                let caps = re2.captures(&s).unwrap();
                let contained_bag_id = caps.get(1).expect("capture failed").as_str().to_string();

                // build the map: bag_id -> [set of bags which contain "bag_id"]
                match containers.get_mut(&contained_bag_id) {
                    Some(hs) => {
                        hs.insert(id.clone());
                    }
                    None => {
                        containers.insert(contained_bag_id.clone(), HashSet::<String>::from_iter([id.clone()]));
                    }
                }
            }
        }
    }
    count_containers(&containers, query_bag)
}

fn solution2(data: &[String], query_bag: &str) -> usize {
    let re1: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)").unwrap();
    let re2: Regex = Regex::new(r"^\s*(\d+) (\w+ \w+) bag").unwrap();

    let mut bags = HashMap::<String, Vec<(String, usize)>>::new();

    for line in data {
        let caps = re1.captures(line).unwrap();
        let id = caps.get(1).expect("capture failed").as_str().to_string();
        let rest = caps.get(2).expect("capture failed").as_str();

        let mut contents = vec![];
        for s in trim_split_on::<String>(rest, ',').unwrap() {
            if s.contains("no other bag") {
                contents.push(("nothing".to_string(), 1));
            } else {
                let caps = re2.captures(&s).unwrap();
                let contained_bag_id = caps.get(2).expect("capture failed").as_str().to_string();
                contents.push((
                    contained_bag_id.clone(),
                    caps.get(1).expect("capture failed").as_str().parse::<usize>().unwrap(),
                ));
            }
        }
        bags.insert(id.clone(), contents);
    }
    count_bags(&bags, query_bag) - 1
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

    writeln!(stdout, "Answer Part 1 = {}", solution1(&puzzle_lines, "shiny gold"))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&puzzle_lines, "shiny gold"))?;

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
        assert_eq!(4, solution1(&data, "shiny gold"));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(142, solution1(&data, "shiny gold"));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(32, solution2(&data, "shiny gold"));
    }

    #[test]
    fn part2_example2() {
        let data = get_data("input-example2");
        assert_eq!(126, solution2(&data, "shiny gold"));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(10219, solution2(&data, "shiny gold"));
    }
}
