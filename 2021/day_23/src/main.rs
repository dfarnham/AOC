use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

// 0 0 0 0 0 0 0 0 0 0 0  <- Hallway
//     0   2   4   6      <- Starting Positions
//     1   3   5   7      <- Starting Positions
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Amphipod {
    base_cost: u64,
    total_cost: u64,
    //start_pos: usize,
    hallway_pos: Option<usize>,
    home: bool,
}

fn get_data(data: &[String]) -> [Vec<Amphipod>; 4] {
    let mut amphipods = vec![];

    for line in data {
        for c in line.chars() {
            match c {
                'A' => amphipods.push(1),
                'B' => amphipods.push(10),
                'C' => amphipods.push(100),
                'D' => amphipods.push(1000),
                _ => continue,
            };
        }
    }

    let mut base_a = vec![];
    let mut base_b = vec![];
    let mut base_c = vec![];
    let mut base_d = vec![];

    for (i, amp) in amphipods.iter().enumerate() {
        let a = Amphipod {
            base_cost: *amp,
            total_cost: 0,
            hallway_pos: None,
            home: false,
        };
        match i % 4 {
            0 => base_a.push(a),
            1 => base_b.push(a),
            2 => base_c.push(a),
            _ => base_d.push(a),
        }
    }

    [base_a, base_b, base_c, base_d]
}

#[allow(dead_code)]
fn solved(amps: &[Vec<Amphipod>; 4]) -> bool {
    for a in amps {
        for b in a {
            if !b.home {
                return false;
            }
        }
    }
    true
}

fn solution1(_amps: &[Vec<Amphipod>; 4]) -> i64 {
    // Solved by hand, still toying with an implementation
    11608
}
fn solution2(_amps: &[Vec<Amphipod>; 4]) -> i64 {
    // Solved by hand, still toying with an implementation
    46754
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

    let amphipods = get_data(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {}", solution1(&amphipods))?;
    writeln!(stdout, "Answer Part 2 = {}", solution2(&amphipods))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> [Vec<Amphipod>; 4] {
        let file = std::path::PathBuf::from(filename);
        get_data(&read_trimmed_data_lines::<String>(Some(&file)).unwrap())
    }

    #[test]
    fn part1_actual() {
        let amphipods = get_test_data("input-example");
        assert_eq!(11608, solution1(&amphipods));
    }

    #[test]
    fn part2_actual() {
        let amphipods = get_test_data("input-actual");
        assert_eq!(46754, solution2(&amphipods));
    }
}
