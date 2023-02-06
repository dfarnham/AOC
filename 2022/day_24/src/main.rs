// influenced by mod_floor() usage in https://github.com/hyper-neutrino/advent-of-code/blob/main/2022/day24p1.py
use general::{get_args, read_data_lines, reset_sigpipe};
use num::integer::{lcm, mod_floor};
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

#[allow(clippy::type_complexity)]
fn get_data(
    data: &[String],
) -> (
    (i64, i64),
    (i64, i64),
    (i64, i64),
    HashSet<(char, i64, i64)>,
) {
    let mut positions = HashSet::new();
    for (i, line) in data.iter().skip(1).enumerate() {
        for (j, c) in line.chars().skip(1).enumerate() {
            if ['<', '>', '^', 'v'].contains(&c) {
                positions.insert((c, i as i64, j as i64));
            }
        }
    }
    let start = (
        -1,
        data[0].chars().position(|c| c == '.').expect("entrance") as i64 - 1,
    );
    let stop = (
        data.len() as i64 - 2,
        data[data.len() - 1]
            .chars()
            .position(|c| c == '.')
            .expect("exit") as i64
            - 1,
    );
    let dim = (data.len() as i64 - 2, data[0].len() as i64 - 2);
    (start, stop, dim, positions)
}

// up, down, left, right, wait
fn udlrw(pos: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)]
        .iter()
        .map(move |(r, c)| (pos.0 + r, pos.1 + c))
}

#[rustfmt::skip]
fn solve(
    start: (i64, i64),
    stop: (i64, i64),
    dim: (i64, i64),
    tm: i64,
    blizzards: &HashSet<(char, i64, i64)>,
) -> Result<i64, Box<dyn Error>> {
    let mut visited = HashSet::<(i64, i64, i64)>::new();

    let mut q = VecDeque::new();
    q.push_back((tm, start));

    let (r, c) = dim;
    let lcm = lcm(r, c);

    let mut bliz = HashMap::new();
    bliz.insert('<', blizzards.iter().filter(|(c, _, _)| c == &'<').map(|(_, r, c)| (*r, *c)).collect::<HashSet<_>>());
    bliz.insert('>', blizzards.iter().filter(|(c, _, _)| c == &'>').map(|(_, r, c)| (*r, *c)).collect::<HashSet<_>>());
    bliz.insert('^', blizzards.iter().filter(|(c, _, _)| c == &'^').map(|(_, r, c)| (*r, *c)).collect::<HashSet<_>>());
    bliz.insert('v', blizzards.iter().filter(|(c, _, _)| c == &'v').map(|(_, r, c)| (*r, *c)).collect::<HashSet<_>>());

    while let Some((t, p)) = q.pop_front() {
        let time = t + 1;

        // udlrw == up, down, left, right, wait
        for (nr, nc) in udlrw(p) {
            if (nr, nc) == stop {
                return Ok(time);
            }

            if (nr, nc) != start && (nr < 0 || nc < 0 || nr >= r || nc >= c) {
                continue;
            }

            let mut fail = false;

            if (nr, nc) != start {
                for (b, tr, tc) in [('<', 0, -1), ('>', 0, 1), ('^', -1, 0), ('v', 1, 0)] {
                    let coord = (mod_floor(nr - tr * time, r), mod_floor(nc - tc * time, c));
                    if bliz[&b].contains(&coord) {
                        fail = true;
                        break;
                    }
                }
            }

            let key = (nr, nc, time % lcm);
            if !fail && !visited.contains(&key) {
                visited.insert(key);
                q.push_back((time, (nr, nc)));
            }
        }
    }
    panic!("no solution")
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let (start, stop, dim, blizzards) = get_data(puzzle_lines);
    solve(start, stop, dim, 0, &blizzards)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let (start, stop, dim, blizzards) = get_data(puzzle_lines);
    let t1 = solve(start, stop, dim, 0, &blizzards)?;
    let t2 = solve(stop, start, dim, t1, &blizzards)?;
    solve(start, stop, dim, t2, &blizzards)
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
        assert_eq!(part1(&puzzle_lines)?, 18);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 238);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 54);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 751);
        Ok(())
    }
}
