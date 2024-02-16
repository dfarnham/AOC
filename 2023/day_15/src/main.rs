use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

fn total_focusing_power(boxes: &[Vec<(String, usize)>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(j, tup)| (i + 1) * (j + 1) * tup.1)
                .sum::<usize>()
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(puzzle_lines[0].split(',').map(hash).sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    //
    // Sequence of letters that indicate the label
    //
    // The label will be immediately followed by a character that indicates the operation to perform:
    // either an equals sign (=) or a dash (-).
    //
    // If the operation character is an equals sign (=), it will be followed by a number indicating the focal length
    //
    let re: Regex = Regex::new(r"^([a-z]+)([-=])(\d+)?").unwrap();
    let mut boxes = vec![Vec::<(String, usize)>::new(); 256];

    for s in puzzle_lines[0].split(',') {
        let captures = re.captures(s).unwrap();
        let label = captures.get(1).expect("capture failed").as_str().to_string();
        let op = captures.get(2).expect("capture failed").as_str().to_string();
        let lenses = &mut boxes[hash(&label)];
        let index = lenses.iter().position(|(lab, _focal_length)| lab == &label);
        match op.as_str() {
            "=" => {
                let focal_length = captures.get(3).expect("capture failed").as_str().parse::<usize>()?;
                // If there is already a lens in the box with the same label, replace the old lens with the new lens:
                // remove the old lens and put the new lens in its place, not moving any other lenses in the box.
                if let Some(i) = index {
                    lenses[i].1 = focal_length;
                } else {
                    lenses.push((label, focal_length));
                }
            }
            "-" => {
                // Go to the relevant box and remove the lens with the given label if it is present in the box.
                // Then, move any remaining lenses as far forward in the box as they can go without changing their order,
                // filling any space made by removing the indicated lens.
                // (If no lens in that box has the given label, nothing happens.)
                if let Some(i) = index {
                    lenses.remove(i);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(total_focusing_power(&boxes))
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
        assert_eq!(part1(&puzzle_lines)?, 1320);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 517965);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 145);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 267372);
        Ok(())
    }
}
