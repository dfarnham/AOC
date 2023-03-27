use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

#[allow(clippy::type_complexity)]
// this turns an array of input lines:
//      10 ORE => 10 A
//      1 ORE => 1 B
//      7 A, 1 B => 1 C
//      7 A, 1 C => 1 D
//      7 A, 1 D => 1 E
//      7 A, 1 E => 1 FUEL
//
// into a HasMmap:
// {
//   "A":    ( 10, [ ("ORE", 10)        ] ),
//   "B":    ( 1,  [ ("ORE", 1)         ] ),
//   "C":    ( 1,  [ ("A", 7), ("B", 1) ] ),
//   "D":    ( 1,  [ ("A", 7), ("C", 1) ] )
//   "E":    ( 1,  [ ("A", 7), ("D", 1) ] ),
//   "FUEL": ( 1,  [ ("A", 7), ("E", 1) ] ),
// }
fn get_data(puzzle_lines: &[String]) -> Result<HashMap<String, (usize, Vec<(String, usize)>)>, Box<dyn Error>> {
    let re = Regex::new(r"(.*)\s+=>\s+(\d+)\s+([A-Z]+)")?;
    let mut reactions = HashMap::new();

    for line in puzzle_lines {
        if let Some(captures) = re.captures(line) {
            // everthing up to " => "
            let quantities = captures.get(1).map(|s| s.as_str().to_string()).unwrap();

            // the formula will be a list of Tuples, e.g. [ ("name", quantity), ... ]
            let mut formula = vec![];
            for n_name in quantities.split(',') {
                let kv: Vec<_> = n_name.split_whitespace().collect();
                formula.push((kv[1].into(), kv[0].parse::<usize>()?));
            }

            // base quantity parsed from: "quantity name"
            // insert into Hash as key=name, value=(quantity, formula)
            reactions.insert(
                captures.get(3).map(|s| s.as_str().into()).unwrap(),
                (captures.get(2).map(|s| s.as_str().parse::<usize>()).unwrap()?, formula),
            );
        }
    }
    Ok(reactions)
}

#[allow(clippy::type_complexity)]
fn ore_count(reactions: &HashMap<String, (usize, Vec<(String, usize)>)>, fuel: usize) -> Result<usize, Box<dyn Error>> {
    let mut req = HashMap::from([("FUEL".to_string(), fuel)]);
    let mut resources = HashMap::from([("FUEL".to_string(), 0)]);

    // occurrence counts
    for (_, formula) in reactions.values() {
        for (chem, _) in formula {
            *resources.entry(chem.into()).or_insert(0) += 1;
        }
    }

    while let Some((r, _)) = resources.clone().into_iter().find(|(_, q)| *q == 0) {
        resources.remove(&r);

        let n = req[&r];
        if r == "ORE" {
            return Ok(n);
        }

        let (lotsz, formula) = &reactions[&r];
        let amt = (n + lotsz - 1) / lotsz;
        for (chem, quantity) in formula {
            *req.entry(chem.into()).or_insert(0) += amt * quantity;
            resources.entry(chem.into()).and_modify(|count| *count -= 1);
        }
    }
    Err("ore_count() failed".into())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let reactions = get_data(puzzle_lines)?;
    ore_count(&reactions, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let reactions = get_data(puzzle_lines)?;
    let ore_goal = 1000000000000;

    let (mut lo, mut hi) = (0, ore_goal);
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if ore_count(&reactions, mid)? > ore_goal {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }

    Ok(lo)
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines)?)?;

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
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 13312);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part1(&puzzle_lines)?, 180697);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part1(&puzzle_lines)?, 2210736);
        Ok(())
    }

    #[test]
    fn part1_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part1(&puzzle_lines)?, 31);
        Ok(())
    }

    #[test]
    fn part1_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
        assert_eq!(part1(&puzzle_lines)?, 165);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 598038);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 82892753);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part2(&puzzle_lines)?, 5586022);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part2(&puzzle_lines)?, 460664);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2269325);
        Ok(())
    }
}
