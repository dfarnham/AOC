use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (HashMap<String, i64>, HashMap<String, Vec<String>>) {
    let mut monkey_values = HashMap::new();
    let mut monkey_exp = HashMap::new();

    for line in data {
        let values = line.split(':').collect::<Vec<&str>>();
        let monkey_name = values[0].to_string();
        let values = values[1].split_whitespace().collect::<Vec<&str>>();
        if values.len() == 1 {
            monkey_values.insert(monkey_name, values[0].parse::<i64>().unwrap());
        } else {
            monkey_exp.insert(
                monkey_name,
                values.into_iter().map(String::from).collect::<Vec<_>>(),
            );
        }
    }
    (monkey_values, monkey_exp)
}

// we have 2 maps
//   1. monkey_values: key="monkey name", value=concrete i64 from an evaluation.
//      (pre-populated with all k,v given to us)
//   2. monkey_expressions: key="monkey name", value=["some monkey name", "+-*/", "some monkey name"]
// returns the value for a "monkey name"
#[rustfmt::skip]
fn solve(
    name: &str,
    monkey_values: &HashMap<String, i64>,
    monkey_expressions: &HashMap<String, Vec<String>>,
) -> HashMap<String, i64> {
    let mut values = monkey_values.clone();
    let expressions = monkey_expressions;

    // loop until name evaluates, could be forever
    while values.get(name).is_none() {
        for (k, v) in expressions {
            if !values.contains_key(k) {
                if let (Some(a), Some(b)) = (values.get(&v[0]), values.get(&v[2])) {
                    let k = k.to_string();
                    match v[1].as_ref() {
                        "+" => { values.insert(k, a + b); }
                        "-" => { values.insert(k, a - b); }
                        "*" => { values.insert(k, a * b); }
                        "/" => { values.insert(k, a / b); }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    values
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let (monkey_values, monkey_expressions) = get_data(puzzle_lines);
    let monkey_values = solve("root", &monkey_values, &monkey_expressions);
    Ok(monkey_values["root"])
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let (values, expressions) = get_data(puzzle_lines);

    // save a copy
    let orig_values = values.clone();

    //
    // this seems wonky with multiple solutions ???
    // i guess the yell "immediately" means take the lowest
    //

    // grab the 2 monkey names which "root" depends on
    let (ma, mb) = match expressions.get("root") {
        Some(v) => (v[0].to_string(), v[2].to_string()),
        _ => unreachable!(),
    };

    // values "ma,mb" are a function of "humn"
    // solve part1 to get initial guestimates
    let mut values = solve("root", &values, &expressions);

    // determine lo, hi ordering
    let (ma, mb) = if values[&ma] < values[&mb] {
        (ma, mb)
    } else {
        (mb, ma)
    };

    // take a swag at bounding where to search for "humn" by
    // increasing values["humn"] looking for ma,mb to cross over
    let mut humn = values["humn"];
    let mut prev_humn = humn;
    while values[&ma] < values[&mb] {
        prev_humn = values["humn"]; // lower bound on exit
        humn *= 2; // increase upper bound

        // evaluate the expressions
        values = orig_values.clone();
        values.insert("humn".to_string(), humn);
        values = solve("root", &values, &expressions);
    }

    //println!("now searching for humn between {prev_humn} - {humn}");

    let (mut lo, mut hi) = (prev_humn, humn);
    let mut mid = 0;
    while values[&ma] != values[&mb] {
        mid = lo + (hi - lo) / 2;

        // evaluate the expressions at `mid`
        values = orig_values.clone();
        values.insert("humn".to_string(), mid);
        values = solve("root", &values, &expressions);

        //println!("tried {mid} distance between root monkeys {ma} - {mb} = {}", values[&ma] - values[&mb]);

        // pick a side
        (lo, hi) = match values[&ma] < values[&mb] {
            true => (mid, hi),
            false => (lo, mid),
        };
    }

    // i'm getting multiple solutions ???
    let mut solutions = vec![];
    for wtf in mid - 10..mid + 10 {
        values = orig_values.clone();
        values.insert("humn".to_string(), wtf);
        values = solve("root", &values, &expressions);
        if values[&ma] == values[&mb] {
            solutions.push(wtf);
        }
    }

    //println!("solutions = {:?}", solutions);

    assert!(!solutions.is_empty());
    Ok(solutions[0])
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
        assert_eq!(part1(&puzzle_lines)?, 152);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 168502451381566);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 301);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 3343167719435);
        Ok(())
    }
}
