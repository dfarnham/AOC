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
    let mut chemicals = HashMap::new();

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
            chemicals.insert(
                captures.get(3).map(|s| s.as_str().into()).unwrap(),
                (captures.get(2).map(|s| s.as_str().parse::<usize>()).unwrap()?, formula),
            );
        }
    }
    Ok(chemicals)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let chemicals = get_data(puzzle_lines)?;
    //println!("{chemicals:?}");
    // {
    //   "A":    ( 10, [ ("ORE", 10)        ] ),
    //   "B":    ( 1,  [ ("ORE", 1)         ] ),
    //   "C":    ( 1,  [ ("A", 7), ("B", 1) ] ),
    //   "D":    ( 1,  [ ("A", 7), ("C", 1) ] )
    //   "E":    ( 1,  [ ("A", 7), ("D", 1) ] ),
    //   "FUEL": ( 1,  [ ("A", 7), ("E", 1) ] ),
    // }

    let mut resources = HashMap::new();
    for (_, formula) in chemicals.values() {
        for (chem, _) in formula.iter() {
            *resources.entry(chem.into()).or_insert(0) += 1;
        }
    }
    resources.insert("FUEL".to_string(), 0);

    let mut req = HashMap::new();
    req.insert("FUEL".to_string(), chemicals["FUEL"].0);

    loop {
        if let Some((c, _)) = resources.clone().into_iter().find(|(_, q)| *q == 0) {
            let n = req[&c];
            if c == "ORE" {
                return Ok(n);
            }

            let (num, formula) = &chemicals[&c];
            let amt = (n + num - 1) / num;
            for (chem, quantity) in formula {
                *req.entry(chem.into()).or_insert(0) += amt * quantity;
                *resources.entry(chem.into()).or_insert(0) -= 1;
            }
            resources.remove(&c);
        } else {
            panic!("no solution");
        }
    }
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let chemicals = get_data(puzzle_lines)?;

    let mut resources = HashMap::new();
    for (_, formula) in chemicals.values() {
        for (chem, _) in formula.iter() {
            *resources.entry(chem.into()).or_insert(0) += 1;
        }
    }
    resources.insert("FUEL".to_string(), 0);

    let mut req = HashMap::new();

    let resources_sav = resources.clone();

    // ************************ HACK ************************
    //
    // this is a pseudo binary-search hack and doesn't generalize but got the right answer
    //
    // compute the ammount of ore for 1 FUEL from part1 and use as a guess for the FUEL quantity
    //
    // then keep adding that ore amount until we overshoot the 1 trillion goal, at which point
    // we back off, and shrink the increment in half.
    //
    // when we overshoot with an increment that's eventually 1 return the previous guess
    //
    // ******************************************************
    let one_ore = part1(puzzle_lines).unwrap();
    let mut guess = 1000000000000 / one_ore;
    req.insert("FUEL".to_string(), guess);

    let mut inc = one_ore;

    loop {
        if let Some((c, _)) = resources.clone().into_iter().find(|(_, q)| *q == 0) {
            let n = req[&c];
            if c == "ORE" {
                if n > 1000000000000 {
                    if inc == 1 {
                        return Ok(guess - 1);
                    } else {
                        guess -= inc;
                        inc /= 2;
                        resources = resources_sav.clone();
                        req.clear();
                        req.insert("FUEL".to_string(), guess);
                    }
                } else {
                    guess += inc;
                    resources = resources_sav.clone();
                    req.clear();
                    req.insert("FUEL".to_string(), guess);
                }
            } else {
                let (num, formula) = &chemicals[&c];
                let amt = (n + num - 1) / num;
                for (chem, quantity) in formula {
                    *req.entry(chem.into()).or_insert(0) += amt * quantity;
                    *resources.entry(chem.into()).or_insert(0) -= 1;
                }
                resources.remove(&c);
            }
        } else {
            panic!("no solution");
        }
    }
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
        assert_eq!(part1(&puzzle_lines)?, 31);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part1(&puzzle_lines)?, 165);
        Ok(())
    }

    #[test]
    fn part1_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part1(&puzzle_lines)?, 13312);
        Ok(())
    }

    #[test]
    fn part1_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part1(&puzzle_lines)?, 180697);
        Ok(())
    }

    #[test]
    fn part1_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
        assert_eq!(part1(&puzzle_lines)?, 2210736);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 598038);
        Ok(())
    }

    #[test]
    fn part2_example3() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example3");
        assert_eq!(part2(&puzzle_lines)?, 82892753);
        Ok(())
    }

    #[test]
    fn part2_example4() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example4");
        assert_eq!(part2(&puzzle_lines)?, 5586022);
        Ok(())
    }

    #[test]
    fn part2_example5() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example5");
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
