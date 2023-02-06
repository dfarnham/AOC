use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::error::Error;
use std::io::{self, Write};

#[derive(Clone, PartialEq)]
enum Cell {
    East,
    South,
    Empty,
}

fn get_image(data: &[String]) -> Array2<Cell> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|c| match c {
                '>' => Cell::East,
                'v' => Cell::South,
                _ => Cell::Empty,
            })
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), Cell::Empty);

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    grid
}

#[allow(dead_code)]
fn display(image: &Array2<Cell>) {
    for row in image.rows() {
        for elem in row {
            match elem {
                Cell::East => print!(">"),
                Cell::South => print!("v"),
                Cell::Empty => print!("."),
            };
        }
        println!();
    }
    println!();
}

fn solution1(image: &Array2<Cell>) -> usize {
    //display(&image);
    let nrows = image.nrows();
    let ncols = image.ncols();
    let mut new_image = image.clone();
    let mut steps = 0;

    loop {
        let mut stuck = true;
        steps += 1;

        for mut row in new_image.rows_mut() {
            // find all the ">." cells in the row and add to "swap" list
            let swaps = (0..row.len())
                .filter(|&i| row[i] == Cell::East && row[(i + 1) % ncols] == Cell::Empty)
                .collect::<Vec<_>>();

            // if the swaps list is ever not empty, we're unstuck (stuck = false)
            stuck &= swaps.is_empty();

            // turn ">." into ".>"
            for i in swaps {
                row[i] = Cell::Empty;
                row[(i + 1) % ncols] = Cell::East;
            }
        }

        for mut col in new_image.columns_mut() {
            // find all the "v." cells in the column and add to "swap" list
            let swaps = (0..col.len())
                .filter(|&j| col[j] == Cell::South && col[(j + 1) % nrows] == Cell::Empty)
                .collect::<Vec<_>>();

            // if the swaps list is ever not empty, we're unstuck (stuck = false)
            stuck &= swaps.is_empty();

            // turn "v." into ".v"
            for j in swaps {
                col[j] = Cell::Empty;
                col[(j + 1) % nrows] = Cell::South;
            }
        }

        // all swap lists were empty
        if stuck {
            break;
        }
    }
    steps
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

    let image = get_image(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {}", solution1(&image))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Array2<Cell> {
        let file = std::path::PathBuf::from(filename);
        get_image(&read_trimmed_data_lines::<String>(Some(&file)).unwrap())
    }

    #[test]
    fn part1_example() {
        let image = get_test_data("input-example");
        assert_eq!(58, solution1(&image));
    }

    #[test]
    fn part1_actual() {
        let image = get_test_data("input-actual");
        assert_eq!(360, solution1(&image));
    }
}
