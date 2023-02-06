use general::{get_args, read_trimmed_data_lines, trim_split_on, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solution1(data: &[String]) -> usize {
    let mut field_ranges = HashMap::new();
    let mut _ticket_values = vec![];
    let mut nearby_tickets = vec![];

    // there are 3 newline separated sections (0 == field ranges, 1 == your ticket, 2 == nearby tickets)
    let mut section = 0;

    // section 0 contains line like "class: 1-3 or 5-7"
    let re: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    for line in data {
        if line.is_empty() {
            section += 1;
            continue;
        }

        if line.starts_with("your ticket:") || line.starts_with("nearby tickets:") {
            continue;
        }

        match section {
            0 => {
                let caps = re.captures(line).unwrap();
                let field_name = caps.get(1).expect("capture failed").as_str().to_string();
                let range1_start = caps.get(2).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range1_end = caps.get(3).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range2_start = caps.get(4).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range2_end = caps.get(5).expect("capture failed").as_str().parse::<usize>().unwrap();

                let predicate =
                    move |n| (range1_start..=range1_end).contains(n) || (range2_start..=range2_end).contains(n);
                field_ranges.insert(field_name, predicate);
            }
            1 => {
                _ticket_values = trim_split_on::<usize>(line, ',').unwrap();
            }
            _ => {
                nearby_tickets.push(trim_split_on::<usize>(line, ',').unwrap());
            }
        }
    }

    nearby_tickets
        .iter()
        .flatten()
        .filter(|n| field_ranges.values().filter(|range_test| range_test(n)).count() == 0)
        .sum()
}

fn solution2(data: &[String]) -> usize {
    let mut field_ranges = HashMap::new();
    let mut ticket_values = vec![];
    let mut nearby_tickets = vec![];

    // there are 3 newline separated sections (0 == field ranges, 1 == your ticket, 2 == nearby tickets)
    let mut section = 0;

    // section 0 contains line like "class: 1-3 or 5-7"
    let re: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    for line in data {
        if line.is_empty() {
            section += 1;
            continue;
        }

        if line.starts_with("your ticket:") || line.starts_with("nearby tickets:") {
            continue;
        }

        match section {
            0 => {
                let caps = re.captures(line).unwrap();
                let field_name = caps.get(1).expect("capture failed").as_str().to_string();
                let range1_start = caps.get(2).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range1_end = caps.get(3).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range2_start = caps.get(4).expect("capture failed").as_str().parse::<usize>().unwrap();
                let range2_end = caps.get(5).expect("capture failed").as_str().parse::<usize>().unwrap();

                field_ranges.insert(field_name, (range1_start..=range1_end, range2_start..=range2_end));
            }
            1 => {
                ticket_values = trim_split_on::<usize>(line, ',').unwrap();
            }
            _ => {
                nearby_tickets.push(trim_split_on::<usize>(line, ',').unwrap());
            }
        }
    }

    // determine the set of field names each number in "your ticket" belongs to
    let mut ranges = vec![];
    for n in &ticket_values {
        let sets = HashSet::<_>::from_iter(
            field_ranges
                .iter()
                .filter(|(_, v)| v.0.contains(n) || v.1.contains(n))
                .map(|(k, _)| k),
        );
        assert!(!sets.is_empty(), "my ticket is invalid");
        ranges.push(sets);
    }

    // compute the intersection (for each position) in the nearby tickets
    'outer: for nearby_ticket in nearby_tickets {
        for (i, n) in nearby_ticket.iter().enumerate() {
            let sets = HashSet::<_>::from_iter(
                field_ranges
                    .iter()
                    .filter(|(_, v)| v.0.contains(n) || v.1.contains(n))
                    .map(|(k, _)| k),
            );

            // invalid ticket number, skip this nearby ticket
            if sets.is_empty() {
                continue 'outer;
            }
            ranges[i] = ranges[i].intersection(&sets).copied().collect();
        }
    }

    // we're done when we have a list of single element sets
    while ranges.iter().filter(|r| r.len() == 1).count() != ranges.len() {
        // determined sets are the sets with a single element
        let determined = ranges
            .iter()
            .filter(|r| r.len() == 1)
            .flat_map(|r| r.iter().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // remove the determined entries from all sets with 2 or more elements
        for d in determined {
            for r in &mut ranges {
                if r.len() > 1 && r.contains(d) {
                    r.remove(d);
                }
            }
        }
    }

    ranges
        .iter()
        .flat_map(|r| r.iter())
        .enumerate()
        .filter(|(_, s)| s.starts_with("departure"))
        .map(|(i, _)| ticket_values[i])
        .product::<usize>()
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
        assert_eq!(71, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(22057, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(1, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(1093427331937, solution2(&data));
    }
}
