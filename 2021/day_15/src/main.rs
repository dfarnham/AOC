use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Array2<u32> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut grid = Array::zeros((0, data[0].len()));

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    grid
}

fn extend_row_right(row: &[u32]) -> Vec<u32> {
    let mut erow = row.to_vec();
    for i in 0..4 {
        erow.extend(row.iter().map(|n| (*n + i) % 9 + 1).collect::<Vec<_>>());
    }
    erow
}

fn get_grid_x5(data: &[String]) -> Array2<u32> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let row = extend_row_right(&get_row(&data[0]));
    let mut grid = Array::zeros((0, row.len()));
    grid.push_row(ArrayView::from(&row)).unwrap();

    // process remaining data[1..]
    for line in &data[1..] {
        grid.push_row(ArrayView::from(&extend_row_right(&get_row(line))))
            .unwrap();
    }
    for i in 1..5 {
        for line in data {
            let mut row = extend_row_right(&get_row(line));
            for _ in 0..i {
                row = row.iter().map(|n| *n % 9 + 1).collect();
            }
            grid.push_row(ArrayView::from(&row)).unwrap();
        }
    }
    grid
}

fn get_adjacents(grid: &Array2<u32>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = (position.0 as i32, position.1 as i32);
    let range = &0..&(grid.nrows() as i32);
    [
        (i, j + 1),
        (i + 1, j),
        (i - 1, j),
        (i, j - 1),
        //(i - 1, j - 1),
        //(i - 1, j + 1),
        //(i + 1, j - 1),
        //(i + 1, j + 1),
    ]
    .iter()
    .filter(|(r, c)| range.contains(&r) && range.contains(&c))
    .map(|(r, c)| (*r as usize, *c as usize))
    .collect::<Vec<(_, _)>>()
}

fn calc_risk(
    current: (usize, usize),
    scored: &HashSet<(usize, usize)>,
    level: usize,
    score: u32,
    best_score: &mut u32,
    risk: &Array2<u32>,
) {
    for pt in get_adjacents(risk, current) {
        let sc = score + risk[[pt.0, pt.1]];
        if sc < *best_score {
            if scored.contains(&pt) {
                *best_score = sc;
            } else if level < 4 {
                // empirically cheating on recursion depth
                calc_risk(pt, scored, level + 1, sc, best_score, risk);
            }
        }
    }
}

fn solution(grid: &Array2<u32>, start: (usize, usize), finish: (usize, usize)) -> u32 {
    let mut scored = HashSet::<(usize, usize)>::from_iter(vec![start]);
    let mut risk = grid.clone();

    for i in 1..grid.nrows() {
        let mut edge = vec![];
        for j in 0..i {
            edge.push((i, j));
            edge.push((j, i));
        }
        edge.push((i, i));

        for pt in edge {
            let mut best_score = u32::MAX;
            let score = risk[[pt.0, pt.1]];
            calc_risk(pt, &scored, 0, score, &mut best_score, &risk);
            risk[[pt.0, pt.1]] = best_score;
            scored.insert(pt);
        }
    }

    //println!("risk =\n{:?}", risk);
    risk[[finish.0, finish.1]] - risk[[start.0, start.1]]
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

    let grid = get_grid(&puzzle_lines);
    writeln!(
        stdout,
        "Answer Part 1 = {}",
        solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1))
    )?;

    let grid = get_grid_x5(&puzzle_lines);
    writeln!(
        stdout,
        "Answer Part 2 = {}",
        solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1))
    )?;

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
        let grid = get_grid(&data);
        assert_eq!(solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1)), 40);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let grid = get_grid(&data);
        assert_eq!(solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1)), 540);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let grid = get_grid_x5(&data);
        assert_eq!(solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1)), 315);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let grid = get_grid_x5(&data);
        assert_eq!(solution(&grid, (0, 0), (grid.nrows() - 1, grid.ncols() - 1)), 2879);
    }
}
