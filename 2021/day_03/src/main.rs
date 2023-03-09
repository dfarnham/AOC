use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use rayon::prelude::*;
use std::error::Error;
use std::io::{self, Write};

// how many bits does the largest value in the dataset occupy
fn nbits(data: &[u32]) -> usize {
    ((*data.iter().max().expect("max() failure") as f32).log2()).round() as usize
}

fn get_gamma_epsilon(data: &[u32]) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    let nbits = nbits(data);
    let masks = (0..nbits).map(|i| 1 << i).collect::<Vec<_>>();
    for mask in masks {
        let count = data.par_iter().filter(|n| (*n & mask) == mask).count();
        // are there more bits "on" than "off" in this position?
        match 2 * count >= data.len() {
            true => gamma |= mask,
            false => epsilon |= mask,
        }
    }
    (gamma, epsilon)
}

fn mask_data<T>(data: &[T], mask: T) -> (Vec<T>, Vec<T>)
where
    T: std::cmp::PartialEq + Copy + std::ops::BitAnd<Output = T>,
{
    let mut masked = vec![];
    let mut unmasked = vec![];
    for n in data {
        match (*n & mask) == mask {
            true => masked.push(*n),
            false => unmasked.push(*n),
        }
    }
    (masked, unmasked)
}

fn get_co2(data: &[u32], mask: u32) -> u32 {
    match data.len() == 1 {
        true => data[0],
        false => {
            let (masked, unmasked) = mask_data(data, mask);
            match unmasked.len() <= masked.len() {
                true => get_co2(&unmasked, mask >> 1),
                false => get_co2(&masked, mask >> 1),
            }
        }
    }
}

fn get_oxy(data: &[u32], mask: u32) -> u32 {
    match data.len() == 1 {
        true => data[0],
        false => {
            let (masked, unmasked) = mask_data(data, mask);
            match masked.len() >= unmasked.len() {
                true => get_oxy(&masked, mask >> 1),
                false => get_oxy(&unmasked, mask >> 1),
            }
        }
    }
}

fn get_oxy_co2(data: &[u32]) -> (u32, u32) {
    // how many bits does the largest value in the dataset occupy
    let nbits = nbits(data);
    let mask = 1 << (nbits - 1);
    (get_oxy(data, mask), get_co2(data, mask))
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    let data = read_trimmed_data_lines::<String>(args.get_one::<std::path::PathBuf>("FILE"))?
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>();

    let (gamma, epsilon) = get_gamma_epsilon(&data);
    //println!("gamma = {}, epsilon = {}", gamma, epsilon);
    writeln!(stdout, "Answer Part 1 = {}", gamma * epsilon)?;

    let (oxy, co2) = get_oxy_co2(&data);
    //println!("oxy = {}, co2 = {}", oxy, co2);
    writeln!(stdout, "Answer Part 2 = {}", oxy * co2)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<u32> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file))
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>();
        data
    }

    #[test]
    fn part1_example() {
        let (gamma, epsilon) = get_gamma_epsilon(&get_data("input-example"));
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn part1_actual() {
        let (gamma, epsilon) = get_gamma_epsilon(&get_data("input-actual"));
        assert_eq!(gamma * epsilon, 1307354);
    }

    #[test]
    fn part2_example() {
        let (oxy, co2) = get_oxy_co2(&get_data("input-example"));
        assert_eq!(oxy, 23);
        assert_eq!(co2, 10);
        assert_eq!(oxy * co2, 230);
    }

    #[test]
    fn part2_actual() {
        let (oxy, co2) = get_oxy_co2(&get_data("input-actual"));
        assert_eq!(oxy * co2, 482500);
    }
}
