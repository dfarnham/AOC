use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

fn compare(left: &(usize, &(Vec<usize>, usize)), right: &(usize, &(Vec<usize>, usize))) -> Ordering {
    let (left_score, right_score) = (left.0, right.0);
    let (left_list, right_list) = (&left.1 .0, &right.1 .0);
    assert_eq!(left_list.len(), right_list.len());

    match left_score.cmp(&right_score) {
        Ordering::Equal => {
            for i in 0..left_list.len() {
                match left_list[i].cmp(&right_list[i]) {
                    Ordering::Equal => continue,
                    sub_ordering => return sub_ordering,
                }
            }
            Ordering::Equal
        }
        ordering => ordering,
    }
}

fn score(num_jokers: usize, hand: &[usize]) -> usize {
    let ctr = hand.iter().collect::<Counter<_>>().most_common_ordered();
    match num_jokers {
        0 => match ctr.len() {
            1 => 6,
            2 => 4.max(ctr[0].1 + 1),
            3 => 2.max(ctr[0].1),
            4 => 1,
            _ => 0,
        },
        1 => match ctr.len() {
            1 => 6,
            2 => 4.max(ctr[0].1 + 2),
            3 => 3,
            _ => 1,
        },
        2 => match ctr.len() {
            1 => 6,
            2 => 5,
            _ => 3,
        },
        3 => 5.max(5+ctr.len()),
        _ => 6,
    }
}

fn get_scored_hands_jokers(hands: &[(Vec<usize>, usize)]) -> Vec<(usize, &(std::vec::Vec<usize>, usize))> {
    let mut scored = vec![];
    for hand in hands.iter() {
        let jokers_removed: Vec<_> = hand.0.iter().filter(|c| **c > 1).copied().collect();
        scored.push((score(5 - jokers_removed.len(), &jokers_removed), hand));
    }
    scored
}

fn solution(puzzle_lines: &[String], p2: bool) -> Result<usize, Box<dyn Error>> {
    let card_value_p1 = |c: char| "..23456789TJQKA".chars().position(|t| t == c).unwrap();
    let card_value_p2 = |c: char| ".J23456789T.QKA".chars().position(|t| t == c).unwrap();

    let mut hands = vec![];
    for line in puzzle_lines {
        let parts: Vec<_> = trim_split_ws::<String>(line)?;
        let card_values: Vec<_> = match p2 {
            true => parts[0].chars().map(card_value_p2).collect(),
            false => parts[0].chars().map(card_value_p1).collect(),
        };
        let bid = parts[1].parse::<usize>()?;
        hands.push((card_values, bid));
    }

    let mut scored_hands: Vec<_> = match p2 {
        true => get_scored_hands_jokers(&hands),
        false => hands.iter().map(|hand| (score(0, &hand.0), hand)).collect(),
    };

    scored_hands.sort_by(compare);
    Ok(scored_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1, hand.1 .1))
        .map(|(rank, bid)| rank * bid)
        .sum())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, false)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, true)
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
        assert_eq!(part1(&puzzle_lines)?, 6440);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 248113761);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 5905);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 246285222);
        Ok(())
    }
}
