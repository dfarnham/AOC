use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (u64, u64) {
    let re = Regex::new(r"Player \d starting position:\s+(\d+)").unwrap();
    (
        re.captures(&data[0])
            .unwrap()
            .get(1)
            .map(|s| s.as_str().parse::<u64>().unwrap())
            .unwrap(),
        re.captures(&data[1])
            .unwrap()
            .get(1)
            .map(|s| s.as_str().parse::<u64>().unwrap())
            .unwrap(),
    )
}

fn updated_pos(pos: u64, sum: u64) -> u64 {
    (pos + sum - 1) % 10 + 1
}

fn solution1(start_pos1: u64, start_pos2: u64) -> u64 {
    // what do you get if you multiply the score of the losing player
    // by the number of times the die was rolled during the game?
    let (score1, score2, roll_count) = simulated1(start_pos1, start_pos2, 1000);
    roll_count * score1.min(score2)
}

fn solution2(start_pos1: u64, start_pos2: u64) -> u64 {
    let mut cache: HashMap<(u64, u64, u64, u64), (u64, u64)> = HashMap::new();
    let mut roll_sums = vec![];

    // 27 combinations
    for i in [1, 2, 3] {
        for j in [1, 2, 3] {
            for k in [1, 2, 3] {
                roll_sums.push([i, j, k].iter().sum::<_>());
            }
        }
    }

    let (wins1, wins2) = all_games(start_pos1, start_pos2, 0, 0, 21, &roll_sums, &mut cache);
    wins1.max(wins2)
}

fn all_games(
    pos1: u64,
    pos2: u64,
    score1: u64,
    score2: u64,
    threshold: u64,
    roll_sums: &[u64],
    cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    match cache.get(&(pos1, pos2, score1, score2)) {
        Some((s1, s2)) => (*s1, *s2),
        None => {
            let mut wins1 = 0;
            let mut wins2 = 0;
            for sum in roll_sums {
                let upos1 = updated_pos(pos1, *sum);
                if score1 + upos1 >= threshold {
                    wins1 += 1;
                } else {
                    let (s1, s2) = all_games(pos2, upos1, score2, score1 + upos1, threshold, roll_sums, cache);
                    wins1 += s2;
                    wins2 += s1;
                }
            }
            cache.insert((pos1, pos2, score1, score2), (wins1, wins2));
            (wins1, wins2)
        }
    }
}

fn simulated1(start_pos1: u64, start_pos2: u64, threshold: u64) -> (u64, u64, u64) {
    let mut pos1 = start_pos1;
    let mut pos2 = start_pos2;
    let mut score1 = 0;
    let mut score2 = 0;
    // the 1-n dice value summed over 3 rolls can be represented
    // in the roll count since the sum is only used in the modulus.
    //
    // the sum of 3 consecutive integers is the midpoint * 3,
    let mut roll_count = 2;

    loop {
        // player 1 roll & move
        pos1 = updated_pos(pos1, 3 * roll_count);
        score1 += pos1;
        if score1 >= threshold {
            break;
        }
        roll_count += 3;

        // player 2 roll & move
        pos2 = updated_pos(pos2, 3 * roll_count);
        score2 += pos2;
        if score2 >= threshold {
            break;
        }
        roll_count += 3;
    }
    (score1, score2, roll_count + 1)
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

    let (pos1, pos2) = get_data(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {}", solution1(pos1, pos2))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(pos1, pos2))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> (u64, u64) {
        let file = std::path::PathBuf::from(filename);
        get_data(&read_trimmed_data_lines::<String>(Some(&file)).unwrap())
    }

    #[test]
    fn part1_example() {
        let (pos1, pos2) = get_test_data("input-example");
        assert_eq!(739785, solution1(pos1, pos2));
    }

    #[test]
    fn part1_actual() {
        let (pos1, pos2) = get_test_data("input-actual");
        assert_eq!(551901, solution1(pos1, pos2));
    }

    #[test]
    fn part2_example() {
        let (pos1, pos2) = get_test_data("input-example");
        assert_eq!(444356092776315, solution2(pos1, pos2));
    }

    #[test]
    fn part2_actual() {
        let (pos1, pos2) = get_test_data("input-actual");
        assert_eq!(272847859601291, solution2(pos1, pos2));
    }
}
