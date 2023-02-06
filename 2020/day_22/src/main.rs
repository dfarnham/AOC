use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::{HashSet, VecDeque};

fn get_data(data: &[String]) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();
    let mut p1 = false;
    for line in data {
        if line.starts_with("Player") {
            p1 = !p1;
            continue;
        } else if line.is_empty() {
            continue;
        }
        let card = line.parse::<usize>().unwrap();
        if p1 {
            player1.push_back(card);
        } else {
            player2.push_back(card);
        }
    }
    (player1, player2)
}

fn solution1(data: &[String]) -> usize {
    let (mut player1, mut player2) = get_data(data);

    while !(player1.is_empty() || player2.is_empty()) {
        match (player1.pop_front(), player2.pop_front()) {
            (Some(p1_card), Some(p2_card)) if p1_card > p2_card => {
                player1.push_back(p1_card);
                player1.push_back(p2_card);
            }
            (Some(p1_card), Some(p2_card)) => {
                player2.push_back(p2_card);
                player2.push_back(p1_card);
            }
            _ => panic!("oops"),
        }
    }

    let winner = match player1.is_empty() {
        true => player2,
        false => player1,
    };

    winner.iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum()
}

fn game(
    player1: &mut VecDeque<usize>,
    player2: &mut VecDeque<usize>,
    seen: &mut HashSet<(usize, VecDeque<usize>)>,
) -> usize {
    while !(player1.is_empty() || player2.is_empty()) {
        if seen.contains(&(1, player1.to_owned())) || seen.contains(&(2, player2.to_owned())) {
            return 1;
        }

        seen.insert((1, player1.to_owned()));
        seen.insert((2, player2.to_owned()));
        match (player1.pop_front(), player2.pop_front()) {
            (Some(p1_card), Some(p2_card)) if player1.len() >= p1_card && player2.len() >= p2_card => {
                match game(
                    &mut player1.iter().take(p1_card).copied().collect::<VecDeque<_>>(),
                    &mut player2.iter().take(p2_card).copied().collect::<VecDeque<_>>(),
                    &mut HashSet::<(usize, VecDeque<usize>)>::new(),
                ) {
                    1 => {
                        player1.push_back(p1_card);
                        player1.push_back(p2_card);
                    }
                    _ => {
                        player2.push_back(p2_card);
                        player2.push_back(p1_card);
                    }
                }
            }
            (Some(p1_card), Some(p2_card)) => {
                if p1_card > p2_card {
                    player1.push_back(p1_card);
                    player1.push_back(p2_card);
                } else {
                    player2.push_back(p2_card);
                    player2.push_back(p1_card);
                }
            }
            _ => panic!("oops"),
        }
    }

    match player1.is_empty() {
        true => 2,
        false => 1,
    }
}

fn solution2(data: &[String]) -> usize {
    let (mut player1, mut player2) = get_data(data);
    game(
        &mut player1,
        &mut player2,
        &mut HashSet::<(usize, VecDeque<usize>)>::new(),
    );
    let winner = match player1.is_empty() {
        true => player2,
        false => player1,
    };

    winner.iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum()
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
        assert_eq!(306, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(33772, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(291, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(35070, solution2(&data));
    }
}
