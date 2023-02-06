use general::{get_args, read_data_lines, reset_sigpipe};
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub struct Monkey {
    id: usize,             // monkey
    items: Vec<u64>,       // Starting items lists your worry level for each item the
                           // monkey is currently holding in the order they will be inspected.
    operation_op: String,  // operation shows how your worry level changes
                           // as that monkey inspects an item.
                           // (An operation like new = old * 5 means that your worry level
                           // after the monkey inspected the item is five times whatever your
                           // worry level was before inspection.)
    operation_var: Option<u64>,
    test: u64,             // Test shows how the monkey uses your worry
                           // level to decide where to throw an item next.
    if_true: usize,        // If true shows what happens with an item if the Test was true.
    if_false: usize,       // If false shows what happens with an item if the Test was false.
}
impl Monkey {
    fn default() -> Self {
        Self {
            id: 0,
            items: vec![],
            operation_op: "".into(),
            operation_var: None,
            test: 0,
            if_true: 0,
            if_false: 0,
        }
    }
    fn calc(&self, item: u64, decrease_worry_factor: u64) -> (usize, u64) {
        let var = match self.operation_var {
            Some(num) => num,
            _ => item, // "old"
        };

        let worry_level = match self.operation_op == "*" {
            true => item * var / decrease_worry_factor,
            _ => (item + var) / decrease_worry_factor,
        };

        match worry_level % self.test {
            0 => (self.if_true, worry_level),
            _ => (self.if_false, worry_level),
        }
    }
}

fn get_monkeys(puzzle_lines: &[String]) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let monkey_re = Regex::new(r"Monkey (\d+):")?;
    let items_re = Regex::new(r"^\s+Starting items: (.*)")?;
    let operation_re = Regex::new(r"^\s+Operation: new = old\s+(\S+)\s+(\S+)")?;
    let test_re = Regex::new(r"^\s+Test: divisible by (\d+)")?;
    let if_true_re = Regex::new(r"^\s+If true: throw to monkey (\d+)")?;
    let if_false_re = Regex::new(r"^\s+If false: throw to monkey (\d+)")?;

    let mut monkeys = vec![];
    let mut monkey = Monkey::default();

    for line in puzzle_lines {
        if monkey_re.is_match(line) {
            let captures = monkey_re.captures(line).unwrap();
            monkey.id = captures
                .get(1)
                .map(|s| s.as_str().parse::<usize>().unwrap())
                .unwrap();
        } else if items_re.is_match(line) {
            let captures = items_re.captures(line).unwrap();
            monkey.items = captures
                .get(1)
                .map(|s| s.as_str())
                .unwrap()
                .split(',')
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        } else if operation_re.is_match(line) {
            let captures = operation_re.captures(line).unwrap();
            monkey.operation_op = captures.get(1).map(|s| s.as_str()).unwrap().to_string();
            monkey.operation_var = captures
                .get(2)
                .map(|s| {
                    if s.as_str() == "old" {
                        None
                    } else {
                        Some(s.as_str().parse::<u64>().unwrap())
                    }
                })
                .unwrap()
        } else if test_re.is_match(line) {
            let captures = test_re.captures(line).unwrap();
            monkey.test = captures
                .get(1)
                .map(|s| s.as_str().parse::<u64>().unwrap())
                .unwrap()
        } else if if_true_re.is_match(line) {
            let captures = if_true_re.captures(line).unwrap();
            monkey.if_true = captures
                .get(1)
                .map(|s| s.as_str().parse::<usize>().unwrap())
                .unwrap()
        } else if if_false_re.is_match(line) {
            let captures = if_false_re.captures(line).unwrap();
            monkey.if_false = captures
                .get(1)
                .map(|s| s.as_str().parse::<usize>().unwrap())
                .unwrap()
        } else if line.trim().is_empty() {
            monkeys.push(monkey.clone())
        }
    }
    monkeys.push(monkey);
    Ok(monkeys)
}

fn inspect(monkeys: &[Monkey], rounds: usize, part: u8) -> Result<usize, Box<dyn Error>> {
    // puzzle part 1: divide by 3
    // puzzle part 2: no decrease (divide by 1)
    let decrease_worry_factor = if part == 1 { 3 } else { 1 };

    // multiply the mod tests for all monkeys, e.g. 13 * 17 * 19 * 23
    let lcm: u64 = monkeys.iter().map(|m| m.test).product();

    let mut monkeys = monkeys.to_vec();
    let mut inspected = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // count items to throw (per monkey)
            inspected[i] += monkeys[i].items.len();

            // monkey business
            for item in monkeys[i].items.clone() {
                let (j, worry_level) = monkeys[i].calc(item, decrease_worry_factor);
                monkeys[j].items.push(worry_level % lcm)
            }

            // all items were thrown
            monkeys[i].items.clear()
        }
    }

    inspected.sort_by(|a, b| b.cmp(a));
    Ok(inspected[0] * inspected[1])
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let m = get_monkeys(puzzle_lines)?;
    inspect(&m, 20, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let m = get_monkeys(puzzle_lines)?;
    inspect(&m, 10000, 2)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

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
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 10605);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 58056);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 2713310158);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 15048718170);
        Ok(())
    }
}
