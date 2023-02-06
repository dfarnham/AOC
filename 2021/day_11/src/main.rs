use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::error::Error;
use std::io::{self, Write};

fn get_adjacents(r: usize, c: usize) -> Vec<(usize, usize)> {
    let (i, j) = (r as i32, c as i32);
    let range = &0..=&9;
    [
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ]
    .iter()
    .filter(|(r, c)| range.contains(&r) && range.contains(&c))
    .map(|(r, c)| (*r as usize, *c as usize))
    .collect::<Vec<(_, _)>>()
}

// increase each energy level and return the flash count
// flash count increases by 1 when an energy level increases
// from 9 to 10 and are only counted once.
fn increase_energy(r: usize, c: usize, energy_levels: &mut Array2<u32>) -> usize {
    match energy_levels[[r, c]] == 9 {
        true => {
            energy_levels[[r, c]] = 10;
            1 + get_adjacents(r, c)
                .iter()
                .map(|(i, j)| increase_energy(*i, *j, energy_levels))
                .sum::<usize>()
        }
        false => {
            if energy_levels[[r, c]] != 10 {
                energy_levels[[r, c]] += 1;
            }
            0
        }
    }
}

fn solution(energy_levels: &mut Array2<u32>) -> (usize, usize) {
    let mut flash_count = 0;
    let mut step = 0;
    let mut all_flashed = 0;
    while step < 100 || all_flashed == 0 {
        step += 1;
        for r in 0..=9 {
            for c in 0..=9 {
                let increase = increase_energy(r, c, energy_levels);
                // count number of flashes for the first 100 steps
                if step <= 100 {
                    flash_count += increase;
                }
            }
        }

        // reset flashed items back to zero
        for elem in energy_levels.iter_mut() {
            if *elem == 10 {
                *elem = 0;
            }
        }

        // if the energy sum is zero, 100% flashed, record the step
        if all_flashed == 0 && energy_levels.sum() == 0 {
            all_flashed = step;
        }
    }

    (flash_count, all_flashed)
}

fn get_energy_levels(data: &[String]) -> Array2<u32> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut energy_levels = Array::zeros((0, data[0].len()));

    // process data[..]
    for line in data {
        energy_levels.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    energy_levels
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

    let (p1, p2) = solution(&mut get_energy_levels(&puzzle_lines));
    writeln!(stdout, "Answer Part 1 = {p1}")?;
    writeln!(stdout, "Answer Part 2 = {p2}")?;

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
        let mut energy_levels = get_energy_levels(&data);
        assert_eq!(solution(&mut energy_levels).0, 1656);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let mut energy_levels = get_energy_levels(&data);
        assert_eq!(solution(&mut energy_levels).0, 1655);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let mut energy_levels = get_energy_levels(&data);
        assert_eq!(solution(&mut energy_levels).1, 195);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let mut energy_levels = get_energy_levels(&data);
        assert_eq!(solution(&mut energy_levels).1, 337);
    }
}
