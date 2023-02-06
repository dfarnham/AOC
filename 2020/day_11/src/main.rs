use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use ndarray::{Array, Array2, ArrayView};

fn get_seats(data: &[String]) -> Array2<Option<bool>> {
    // row parsing rules for lines in data
    let get_row = |s: &str| {
        s.chars()
            .map(|c| match c == '.' {
                true => None,
                _ => Some(false),
            })
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), None);

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    grid
}

#[rustfmt::skip]
fn occupied_adjacent(seats: &Array2<Option<bool>>, i: usize, j: usize) -> usize {
    let i = i as i32;
    let j = j as i32;

    [(i-1, j-1), (i-1, j), (i-1, j+1),
     (i,   j-1),           (i,   j+1),
     (i+1, j-1), (i+1, j), (i+1, j+1)].iter()
    .filter(|(r, c)| {
        r >= &0
            && c >= &0
            && r < &(seats.nrows() as i32)
            && c < &(seats.ncols() as i32)
            && seats[[*r as usize, *c as usize]] == Some(true)
    })
    .count()
}

fn occupied_sight(seats: &Array2<Option<bool>>, i: usize, j: usize) -> usize {
    let mut cnt = 0;
    for (x, y) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let mut r = i as i32 + x;
        let mut c = j as i32 + y;
        while r >= 0 && c >= 0 && r < seats.nrows() as i32 && c < seats.ncols() as i32 {
            match seats[[r as usize, c as usize]] {
                None => {
                    r += x;
                    c += y;
                }
                Some(true) => {
                    cnt += 1;
                    break;
                }
                Some(false) => break,
            }
        }
    }
    cnt
}

#[allow(dead_code)]
fn display(seats: &Array2<Option<bool>>) {
    for i in 0..seats.nrows() {
        for j in 0..seats.ncols() {
            match seats[[i, j]] {
                Some(true) => print!("#"),
                Some(false) => print!("L"),
                None => print!("."),
            }
        }
        println!();
    }
    println!()
}

fn solution1(seats: &Array2<Option<bool>>) -> usize {
    let mut updated = seats.clone();
    loop {
        let prev = updated.clone();
        //display(&prev);
        for i in 0..seats.nrows() {
            for j in 0..seats.ncols() {
                if prev[[i, j]].is_none() {
                    continue;
                }
                let occupied = occupied_adjacent(&prev, i, j);
                match prev[[i, j]] {
                    Some(true) => {
                        if occupied >= 4 {
                            updated[[i, j]] = Some(false)
                        }
                    }
                    Some(false) => {
                        if occupied == 0 {
                            updated[[i, j]] = Some(true)
                        }
                    }
                    _ => (),
                }
            }
        }
        if updated == prev {
            return prev.into_iter().filter(|e| *e == Some(true)).count();
        }
    }
}

fn solution2(seats: &Array2<Option<bool>>) -> usize {
    let mut updated = seats.clone();
    loop {
        let prev = updated.clone();
        //display(&prev);
        for i in 0..seats.nrows() {
            for j in 0..seats.ncols() {
                if prev[[i, j]].is_none() {
                    continue;
                }
                let occupied = occupied_sight(&prev, i, j);
                match prev[[i, j]] {
                    Some(true) => {
                        if occupied >= 5 {
                            updated[[i, j]] = Some(false)
                        }
                    }
                    Some(false) => {
                        if occupied == 0 {
                            updated[[i, j]] = Some(true)
                        }
                    }
                    _ => (),
                }
            }
        }
        if updated == prev {
            return prev.into_iter().filter(|e| *e == Some(true)).count();
        }
    }
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

    let seats = get_seats(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&seats))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&seats))?;

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
        let seats = get_seats(&data);
        assert_eq!(37, solution1(&seats));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let seats = get_seats(&data);
        assert_eq!(2321, solution1(&seats));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let seats = get_seats(&data);
        assert_eq!(26, solution2(&seats));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let seats = get_seats(&data);
        assert_eq!(2102, solution2(&seats));
    }
}
