use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{s, Array, Array2, ArrayView};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (Vec<bool>, Array2<bool>) {
    let algorithm = data[0].chars().map(|c| c == '#').collect::<Vec<_>>();
    assert_eq!(algorithm.len(), 512);
    let image = get_image(&data[2..]);
    (algorithm, image)
}

fn get_image(data: &[String]) -> Array2<bool> {
    // row parsing rules for data[String]
    let get_row = |s: &str| s.chars().map(|c| c == '#').collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), false);

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    grid
}

fn pixel(i: usize, j: usize, image: &Array2<bool>, algorithm: &[bool]) -> bool {
    let mut neighbors = [false; 9];

    // row above i,j
    if i != 0 {
        if j != 0 {
            neighbors[0] = image[[i - 1, j - 1]];
        }
        neighbors[1] = image[[i - 1, j]];
        if j != image.ncols() - 1 {
            neighbors[2] = image[[i - 1, j + 1]];
        }
    }

    // row containing i,j
    if j != 0 {
        neighbors[3] = image[[i, j - 1]];
    }
    neighbors[4] = image[[i, j]];
    if j != image.ncols() - 1 {
        neighbors[5] = image[[i, j + 1]];
    }

    // row below i,j
    if i != image.nrows() - 1 {
        if j != 0 {
            neighbors[6] = image[[i + 1, j - 1]];
        }
        neighbors[7] = image[[i + 1, j]];
        if j != image.ncols() - 1 {
            neighbors[8] = image[[i + 1, j + 1]];
        }
    }

    let mut index: u16 = 0;
    for state in neighbors {
        index <<= 1;
        index |= match state {
            true => 1,
            false => 0,
        };
    }

    algorithm[index as usize]
}

fn enhance(image: &Array2<bool>, algorithm: &[bool]) -> Array2<bool> {
    let mut image_padded = Array::from_elem((image.nrows() + 6, image.ncols() + 6), false);
    let mut enhanced = image_padded.clone();

    for i in 0..image.nrows() {
        for j in 0..image.ncols() {
            image_padded[[i + 3, j + 3]] = image[[i, j]];
        }
    }

    for i in 0..image_padded.nrows() {
        for j in 0..image_padded.ncols() {
            enhanced[[i, j]] = pixel(i, j, &image_padded, algorithm);
        }
    }
    enhanced
}

fn solution(image: &Array2<bool>, algorithm: &[bool], n: usize) -> usize {
    let mut new_image = image.clone();
    for i in 0..n {
        /*
        new_image = match i % 2 == 0 {
            true => enhance(&new_image, algorithm),
            false => enhance(&new_image, algorithm)
                .slice(s![4..(new_image.nrows() + 2), 4..(new_image.ncols() + 2)])
                .to_owned(),
        };
        */
        new_image = enhance(&new_image, algorithm);
        if i % 2 == 1 {
            new_image = new_image
                .slice(s![4..(new_image.nrows() - 4), 4..(new_image.ncols() - 4)])
                .to_owned();
        }
    }
    new_image.into_iter().filter(|p| *p).count()
}

#[allow(dead_code)]
fn display(image: &Array2<bool>) {
    for i in 0..image.nrows() {
        for j in 0..image.ncols() {
            match image[[i, j]] {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!();
    }
    println!();
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

    let (algorithm, image) = get_data(&puzzle_lines);

    writeln!(stdout, "Answer Part 1 = {}", solution(&image, &algorithm, 2))?;
    writeln!(stdout, "Answer Part 2 = {}", solution(&image, &algorithm, 50))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_test_data("input-example");
        let (algorithm, image) = get_data(&data);
        assert_eq!(35, solution(&image, &algorithm, 2));
    }

    #[test]
    fn part1_actual() {
        let data = get_test_data("input-actual");
        let (algorithm, image) = get_data(&data);
        assert_eq!(5432, solution(&image, &algorithm, 2));
    }

    #[test]
    fn part2_example() {
        let data = get_test_data("input-example");
        let (algorithm, image) = get_data(&data);
        assert_eq!(3351, solution(&image, &algorithm, 50));
    }

    #[test]
    fn part2_actual() {
        let data = get_test_data("input-actual");
        let (algorithm, image) = get_data(&data);
        assert_eq!(16016, solution(&image, &algorithm, 50));
    }
}
