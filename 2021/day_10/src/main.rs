use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn get_solutions(data: &[String]) -> (u64, u64) {
    let mut illegal = Counter::<char, u64>::new();
    let mut scores = vec![];
    for line in data {
        let mut stack = vec![];
        let mut corrupt_line = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let p = stack.pop();
                    if p.is_none()
                        || p == Some('(') && c != ')'
                        || p == Some('[') && c != ']'
                        || p == Some('{') && c != '}'
                        || p == Some('<') && c != '>'
                    {
                        illegal[&c] += 1;
                        corrupt_line = true;
                        break;
                    }
                }
                _ => panic!("unknown char: {c}"),
            }
        }

        if !corrupt_line {
            let mut score = 0;
            while let Some(c) = stack.pop() {
                score *= 5;
                match c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("unexpected char: {c}"),
                }
            }
            scores.push(score);
        }
    }
    assert_eq!(scores.len() % 2, 1, "scores must be and odd number: {}", scores.len());
    scores.sort_unstable();
    (
        3 * illegal[&')'] + 57 * illegal[&']'] + 1197 * illegal[&'}'] + 25137 * illegal[&'>'],
        scores[scores.len() / 2],
    )
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

    let (p1, p2) = get_solutions(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {p1}")?;
    writeln!(stdout, "Answer Part 2 = {p2}")?;

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
        assert_eq!(get_solutions(&data).0, 26397);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(get_solutions(&data).0, 464991);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(get_solutions(&data).1, 288957);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(get_solutions(&data).1, 3662008566);
    }
}
