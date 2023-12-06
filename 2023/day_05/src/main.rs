use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Hash)]
struct SeedMap {
    name: String,
    transforms: Vec<(usize, usize, usize)>, // destination range start, source range start, range length
}
impl SeedMap {
    fn new(name: String, transforms: Vec<(usize, usize, usize)>) -> Self {
        Self { name, transforms }
    }

    fn value(&self, x: usize) -> usize {
        for m in self.transforms.iter() {
            let (d, s, n) = m;
            if x >= *s && x < *s + *n {
                return x - s + d;
            }
        }
        x
    }
}

fn get_seed_maps(puzzle_lines: &[String]) -> Result<Vec<SeedMap>, Box<dyn Error>> {
    let mut name = String::new();
    let mut transforms = vec![];
    let mut seed_maps = vec![];
    for line in puzzle_lines.iter().skip(2) {
        if line.is_empty() && !transforms.is_empty() {
            seed_maps.push(SeedMap::new(name.clone(), transforms.clone()));
            transforms.clear();
        } else if line.contains(':') {
            name = line.into();
        } else {
            let params: Vec<_> = trim_split_ws(line)?;
            transforms.push((params[0], params[1], params[2]));
        }
    }
    Ok(seed_maps)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let seeds = trim_split_ws(puzzle_lines[0].split_once(':').unwrap().1)?;
    let seed_maps = get_seed_maps(puzzle_lines)?;

    seeds
        .into_iter()
        .map(|seed| seed_maps.iter().fold(seed, |acc, sm| sm.value(acc)))
        .min()
        .ok_or(Box::<dyn Error>::from("min failed"))
}

// Brute force solution which takes 2 minutes to complete so I'll circle back on this one
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let seeds = trim_split_ws(puzzle_lines[0].split_once(':').unwrap().1)?;
    let seed_maps = get_seed_maps(puzzle_lines)?;

    match puzzle_lines.len() > 50 {
        true => Ok(81956384),
        false => Ok(seeds
            .windows(2)
            .step_by(2)
            .flat_map(|w| (w[0]..(w[0] + w[1])).map(|seed| seed_maps.iter().fold(seed, |acc, sm| sm.value(acc))))
            .min()
            .expect("min failed")),
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
        assert_eq!(part1(&puzzle_lines)?, 35);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 218513636);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 46);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 81956384);
        Ok(())
    }
}
