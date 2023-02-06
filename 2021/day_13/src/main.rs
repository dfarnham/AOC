use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{s, Array2};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (Array2<u32>, Vec<i32>) {
    let mut folds = vec![];
    let mut x = vec![];
    let mut y = vec![];

    for line in data {
        match line.contains(',') {
            true => {
                let points = line
                    .trim()
                    .split(',')
                    .map(|s| s.to_string().parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(points.len(), 2, "expected 2 points: {points:?}");
                x.push(points[0]);
                y.push(points[1]);
            }
            false => {
                if !line.is_empty() {
                    let parts = line.trim().split('=').map(|s| s.into()).collect::<Vec<String>>();
                    assert_eq!(parts.len(), 2, "expected 2 parts: {parts:?}");
                    match parts[0].as_ref() {
                        "fold along x" => folds.push(-parts[1].parse::<i32>().unwrap()),
                        "fold along y" => folds.push(parts[1].parse::<i32>().unwrap()),
                        _ => panic!("unknown instruction: {}", parts[0]),
                    };
                }
            }
        }
    }

    let xmax = *x.iter().max().expect("xmax failure");
    let ymax = *y.iter().max().expect("ymax max failure");

    // create a new Array2
    let mut paper = Array2::zeros((xmax + 1, ymax + 1));

    for (i, j) in x.into_iter().zip(y.into_iter()) {
        paper[[i, j]] = 1;
    }
    (paper, folds)
}

fn fold_up(paper: &Array2<u32>, pos: usize) -> Array2<u32> {
    // copy over elements from paper not being folded
    let mut folded = paper
        .slice(s![0..paper.nrows(), 0..pos.max(paper.ncols() - pos - 1)])
        .to_owned();

    // update with folded items from paper
    // if the sum > 0 it is set to 1
    for i in 0..folded.nrows() {
        for (c, j) in ((pos + 1)..paper.ncols()).enumerate() {
            let a = j - 2 - 2 * c;
            folded[[i, a]] = 1.min(folded[[i, a]] + paper[[i, j]]);
        }
    }
    folded
}

fn fold_left(paper: &Array2<u32>, pos: usize) -> Array2<u32> {
    // copy over elements from paper not being folded
    let mut folded = paper
        .slice(s![0..pos.max(paper.nrows() - pos - 1), 0..paper.ncols()])
        .to_owned();

    // update with folded items from paper
    // if the sum > 0 it is set to 1
    for (c, i) in ((pos + 1)..paper.nrows()).enumerate() {
        let a = i - 2 - 2 * c;
        for j in 0..folded.ncols() {
            folded[[a, j]] = 1.min(folded[[a, j]] + paper[[i, j]]);
        }
    }
    folded
}

fn get_message(paper: &Array2<u32>) -> String {
    let mut message = "".to_string();
    for row in paper.t().rows() {
        for elem in row {
            match elem {
                0 => message += " ",
                _ => message += "#",
            };
        }
        message += "\n";
    }
    message
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

    let (mut paper, instructions) = get_data(&puzzle_lines);

    // instructions are < 0 for "left", > 0 for "up"
    for instruction in &instructions[0..1] {
        paper = match instruction < &0 {
            true => fold_left(&paper, instruction.unsigned_abs() as usize),
            false => fold_up(&paper, *instruction as usize),
        };
    }

    writeln!(stdout, "Answer Part 1 = {}", paper.sum())?;

    for instruction in &instructions[1..] {
        paper = match instruction < &0 {
            true => fold_left(&paper, instruction.unsigned_abs() as usize),
            false => fold_up(&paper, *instruction as usize),
        };
    }

    writeln!(stdout, "Answer Part 2 =\n{}", get_message(&paper))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_testdata(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_testdata("input-example");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions[0..1] {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        assert_eq!(paper.sum(), 17);
    }

    #[test]
    fn part1_actual() {
        let data = get_testdata("input-actual");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions[0..1] {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        assert_eq!(paper.sum(), 790);
    }

    #[test]
    fn part2_example() {
        let data = get_testdata("input-example");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        let message = get_message(&paper);
        let expected = "#####\n#   #\n#   #\n#   #\n#####\n     \n     \n";
        assert_eq!(message, expected);
    }

    #[test]
    fn part2_actual() {
        let data = get_testdata("input-actual");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        let message = get_message(&paper);
        let expected = "###   ##  #  # #### ###  ####   ##  ##  \n#  # #  # #  #    # #  # #       # #  # \n#  # #    ####   #  ###  ###     # #    \n###  # ## #  #  #   #  # #       # #    \n#    #  # #  # #    #  # #    #  # #  # \n#     ### #  # #### ###  #     ##   ##  \n";
        assert_eq!(message, expected);
    }
}
