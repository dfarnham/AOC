use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn solution1(data: &[String]) -> usize {
    let mut facing = 0; // 0, 90, 180, 270
    let mut position = (0, 0);

    for line in data {
        let mut chars = line.chars();

        let action = match chars.next() {
            // transform a Forward action into a new Direction based on facing angle
            Some('F') => match facing {
                90 => 'S',
                180 => 'W',
                270 => 'N',
                _ => 'E',
            },
            Some(c) => c,
            _ => panic!("no char"),
        };
        let value = chars.collect::<String>().parse::<i32>().expect("cant parse");

        match action {
            'N' => {
                position.1 += value;
            }
            'S' => {
                position.1 -= value;
            }
            'E' => {
                position.0 += value;
            }
            'W' => {
                position.0 -= value;
            }
            'L' => {
                facing = (facing + 360 - value) % 360;
            }
            'R' => {
                facing = (facing + value) % 360;
            }
            _ => (),
        }
    }
    (position.0.abs() + position.1.abs()) as usize
}

fn solution2(data: &[String]) -> usize {
    let mut waypoint = (10, 1);
    let mut position = (0, 0);

    for line in data {
        let mut chars = line.chars();

        let action = chars.next().expect("a char");
        let value = chars.collect::<String>().parse::<i32>().expect("cant parse");

        match action {
            'F' => {
                position.0 += value * waypoint.0;
                position.1 += value * waypoint.1;
            }
            'N' => {
                waypoint.1 += value;
            }
            'S' => {
                waypoint.1 -= value;
            }
            'E' => {
                waypoint.0 += value;
            }
            'W' => {
                waypoint.0 -= value;
            }
            'L' => {
                waypoint = match value {
                    90 => (-waypoint.1, waypoint.0),
                    180 => (-waypoint.0, -waypoint.1),
                    270 => (waypoint.1, -waypoint.0),
                    _ => waypoint,
                };
            }
            'R' => {
                waypoint = match value {
                    90 => (waypoint.1, -waypoint.0),
                    180 => (-waypoint.0, -waypoint.1),
                    270 => (-waypoint.1, waypoint.0),
                    _ => waypoint,
                };
            }
            _ => (),
        }
    }
    (position.0.abs() + position.1.abs()) as usize
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
        assert_eq!(25, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(381, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(286, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(28591, solution2(&data));
    }
}
