use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};

fn get_solution2(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let min = **counts.keys().min().expect("min() failure");
    let max = **counts.keys().max().expect("max() failure");

    let mut best: Option<(_, _)> = None;
    let sum_to_n = |n| (n * (n + 1) / 2) as usize;
    for pos in min..=max {
        let left_cost = (min..pos)
            .map(|i| counts[&i] * sum_to_n(pos - i))
            .sum::<usize>();
        let right_cost = (pos..=max)
            .map(|i| counts[&i] * sum_to_n(i - pos))
            .sum::<usize>();
        match left_cost + right_cost {
            n if best.is_none() || n < best.unwrap().0 => best = Some((n, pos)),
            _ => break,
        }
    }
    best.expect("no solution chosen")
}

fn get_solution1(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let mut left_ptr = **counts.keys().min().expect("min() failure");
    let mut right_ptr = **counts.keys().max().expect("max() failure");

    let mut left_mass = counts[&left_ptr];
    let mut rigt_mass = counts[&right_ptr];
    let mut cost = 0;
    while left_ptr != right_ptr {
        match left_mass < rigt_mass {
            true => {
                cost += left_mass;
                left_ptr += 1;
                left_mass += counts[&left_ptr];
            }
            false => {
                cost += rigt_mass;
                right_ptr -= 1;
                rigt_mass += counts[&right_ptr];
            }
        }
    }
    //println!("left_mass = {}, rigt_mass = {}, cost = {}", left_mass, rigt_mass, cost);
    (cost, left_ptr)
}

#[allow(unused_variables)]
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

    let data = trim_split_on::<u32>(&puzzle_lines[0], ',')?;

    let (fuel_consumption, position) = get_solution1(&data);
    //println!("Position = {}", position);
    writeln!(stdout, "Answer Part 1 = {fuel_consumption}")?;

    let (fuel_consumption, position) = get_solution2(&data);
    //println!("Position = {}", position);
    writeln!(stdout, "Answer Part 2 = {fuel_consumption}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datapoints(filename: &str) -> Vec<u32> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        let line = &data[0];
        trim_split_on::<u32>(line, ',').unwrap()
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 37);
        assert_eq!(position, 2);
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 349769);
        assert_eq!(position, 331);
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 168);
        assert_eq!(position, 5);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 99540554);
        assert_eq!(position, 479);
    }
}
