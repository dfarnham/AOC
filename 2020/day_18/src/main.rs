use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

#[derive(PartialEq)]
enum Operator {
    Add,
    Mul,
}

fn expr(e: &str) -> Option<(usize, i64)> {
    let mut operand: Option<(usize, i64)> = None;
    let mut operator = None;
    let tokens = e.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            '(' => match expr(&e[i + 1..]) {
                Some(result) => {
                    i += result.0;
                    operand = match operator {
                        Some(Operator::Add) => Some((i, operand.unwrap().1 + result.1)),
                        Some(Operator::Mul) => Some((i, operand.unwrap().1 * result.1)),
                        None => Some((i, result.1)),
                    }
                }
                None => unreachable!(),
            },
            ')' => {
                return Some((i + 1, operand.unwrap().1));
            }
            '+' => {
                operator = Some(Operator::Add);
            }
            '*' => {
                operator = Some(Operator::Mul);
            }
            c if "0123456789".contains(c) => {
                let n = "0123456789".find(c).unwrap() as i64;
                operand = match operator {
                    Some(Operator::Add) => Some((i, operand.unwrap().1 + n)),
                    Some(Operator::Mul) => Some((i, operand.unwrap().1 * n)),
                    None => Some((i, n)),
                }
            }
            _ => {}
        }
        i += 1;
    }
    operand
}

fn expr2(e: &str) -> (usize, i64) {
    let mut operator = vec![];
    let mut operand = vec![];

    let tokens = e.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            '(' => {
                let result = expr2(&e[i + 1..]);
                i += result.0;
                if operator.is_empty() || operator[operator.len() - 1] == Operator::Mul {
                    operand.push(result.1);
                } else {
                    let a = operand.pop().unwrap();
                    operand.push(a + result.1);
                    operator.pop();
                }
            }
            ')' => {
                i += 1;
                break;
            }
            '*' => {
                if !operator.is_empty() && operator[operator.len() - 1] == Operator::Mul && operand.len() > 1 {
                    let a = operand.pop().unwrap();
                    let b = operand.pop().unwrap();
                    operand.push(a * b);
                } else {
                    operator.push(Operator::Mul);
                }
            }
            '+' => {
                operator.push(Operator::Add);
            }
            c if "0123456789".contains(c) => {
                let n = "0123456789".find(c).unwrap() as i64;
                if operator.is_empty() || operator[operator.len() - 1] == Operator::Mul {
                    operand.push(n);
                } else {
                    let a = operand.pop().unwrap();
                    operand.push(a + n);
                    operator.pop();
                }
            }
            _ => {}
        }
        i += 1;
    }
    match operand.len() {
        0 => panic!("oops, empty operand"),
        1 => (i, operand[0]),
        _ => match operator[0] {
            Operator::Add => (i, operand[0] + operand[1]),
            Operator::Mul => (i, operand[0] * operand[1]),
        },
    }
}

fn solution1(data: &[String]) -> i64 {
    data.iter().map(|e| expr(e).unwrap().1).sum()
}

fn solution2(data: &[String]) -> i64 {
    data.iter().map(|e| expr2(e).1).sum()
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
    fn part1_example2() {
        assert_eq!(51, solution1(&["1 + (2 * 3) + (4 * (5 + 6))".to_string()]));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(26, solution1(&["2 * 3 + (4 * 5)".to_string()]));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(437, solution1(&["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()]));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(
            12240,
            solution1(&["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()])
        )
    }

    #[test]
    fn part1_example6() {
        assert_eq!(
            13632,
            solution1(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()])
        );
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(23507031841020, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(231, solution2(&data));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(51, solution2(&["1 + (2 * 3) + (4 * (5 + 6))".to_string()]));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(46, solution2(&["2 * 3 + (4 * 5)".to_string()]));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(1445, solution2(&["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()]));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(
            669060,
            solution2(&["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()])
        )
    }

    #[test]
    fn part2_example6() {
        assert_eq!(
            23340,
            solution2(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()])
        );
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(218621700997826, solution2(&data));
    }
}
