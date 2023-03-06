use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

#[allow(clippy::type_complexity)]
fn get_data(puzzle_lines: &[String]) -> Result<Vec<(i64, i64, i64)>, Box<dyn Error>> {
    let re = Regex::new(r"x=(-?\d+).*?y=(-?\d+).*?z=(-?\d+)")?;
    let mut moons = vec![];

    for line in puzzle_lines {
        if let Some(captures) = re.captures(line) {
            let x = captures.get(1).map(|s| s.as_str().parse::<i64>()).unwrap()?;
            let y = captures.get(2).map(|s| s.as_str().parse::<i64>()).unwrap()?;
            let z = captures.get(3).map(|s| s.as_str().parse::<i64>()).unwrap()?;
            moons.push((x, y, z));
        }
    }
    Ok(moons)
}

fn update_pos_velocity(moons: &mut [(i64, i64, i64)], velocity: &mut [(i64, i64, i64)]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            // x
            match moons[i].0.cmp(&moons[j].0) {
                Ordering::Greater => {
                    velocity[i].0 -= 1;
                    velocity[j].0 += 1;
                }
                Ordering::Less => {
                    velocity[i].0 += 1;
                    velocity[j].0 -= 1;
                }
                Ordering::Equal => {}
            };
            // y
            match moons[i].1.cmp(&moons[j].1) {
                Ordering::Greater => {
                    velocity[i].1 -= 1;
                    velocity[j].1 += 1;
                }
                Ordering::Less => {
                    velocity[i].1 += 1;
                    velocity[j].1 -= 1;
                }
                Ordering::Equal => {}
            };
            // z
            match moons[i].2.cmp(&moons[j].2) {
                Ordering::Greater => {
                    velocity[i].2 -= 1;
                    velocity[j].2 += 1;
                }
                Ordering::Less => {
                    velocity[i].2 += 1;
                    velocity[j].2 -= 1;
                }
                Ordering::Equal => {}
            };
        }
    }

    // positions
    for i in 0..moons.len() {
        moons[i].0 += velocity[i].0;
        moons[i].1 += velocity[i].1;
        moons[i].2 += velocity[i].2;
    }
}

fn part1(puzzle_lines: &[String], steps: usize) -> Result<i64, Box<dyn Error>> {
    let mut moons = get_data(puzzle_lines)?;
    let mut velocity = vec![(0, 0, 0); moons.len()];

    for _ in 0..steps {
        update_pos_velocity(&mut moons, &mut velocity);
    }

    Ok(moons
        .iter()
        .zip(velocity.iter())
        //                     potential energy                      kinetic energy
        .map(|(m, v)| (m.0.abs() + m.1.abs() + m.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
        .sum())
}

//#[rustfmt::skip]
fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut moons = get_data(puzzle_lines)?;
    let mut velocity = vec![(0, 0, 0); moons.len()];
    let mut visited = vec![HashSet::new(); 3];
    let mut cycles = vec![None; 3];

    // cycles is a vector initialized to None
    // loop until all cycles are Some(iteration count)
    while cycles.iter().any(|c| c.is_none()) {
        // 0 == x
        // 1 == y
        // 2 == z
        //
        // this is simply creating/inserting hash keys on every moon & velocity update.
        //
        // each axis has an associated "visited" HashSet.
        // the hash keys are inserted on each axis until a collision occurs
        // the length of the HashSet at first collision is the cycle count

        for i in 0..3 {
            if cycles[i].is_none() {
                // slice the moons across the x, y, or z axis and build a hash key
                let query: Vec<_> = match i {
                    0 => moons.iter().map(|m| m.0).zip(velocity.iter().map(|v| v.0)).collect(),
                    1 => moons.iter().map(|m| m.1).zip(velocity.iter().map(|v| v.1)).collect(),
                    2 => moons.iter().map(|m| m.2).zip(velocity.iter().map(|v| v.2)).collect(),
                    _ => panic!("wtf"),
                };

                // insert keys until a collision
                if visited[i].contains(&query) {
                    cycles[i] = Some(visited[i].len());
                    visited[i].clear(); // done with this data
                } else {
                    visited[i].insert(query);
                }
            }
        }

        update_pos_velocity(&mut moons, &mut velocity);
    }

    //println!("{cycles:?}");
    // input-example: [Some(18), Some(28), Some(44)]
    // input-example2: [Some(2028), Some(5898), Some(4702)]

    // return LCM over cycles
    Ok(cycles
        .iter()
        .map(|c| c.unwrap())
        .reduce(num_integer::lcm)
        .expect("lcm error"))
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

    // true == input-example
    let steps = match puzzle_lines[0] == "<x=-1, y=0, z=2>" {
        true => 10,
        false => 1000,
    };
    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines, steps)?)?;
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
        assert_eq!(part1(&puzzle_lines, 10)?, 179);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part1(&puzzle_lines, 100)?, 1940);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines, 1000)?, 8044);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 2772);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part2(&puzzle_lines)?, 4686774924);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 362375881472136);
        Ok(())
    }
}
