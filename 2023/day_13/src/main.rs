use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Array2<char>, Box<dyn Error>> {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), '.');

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }

    Ok(grid)
}

fn reflection_indices(grid: &Array2<char>, p1_solution: Option<(usize, usize)>) -> (usize, usize) {
    let rowvec = grid
        .rows()
        .into_iter()
        .map(|row| row.iter().copied().collect::<String>())
        .collect::<Vec<_>>();
    let colvec = grid
        .columns()
        .into_iter()
        .map(|col| col.iter().copied().collect::<String>())
        .collect::<Vec<_>>();

    let row_fold_candidates: Vec<_> = rowvec
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .map(|(i, _)| i)
        .collect();

    let col_fold_candidates: Vec<_> = colvec
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .map(|(i, _)| i)
        .collect();

    let rmid = rowvec.len() / 2;
    let cmid = colvec.len() / 2;

    let mut h = 0;
    for index in row_fold_candidates {
        if let Some(tup) = p1_solution {
            // skip part1 solution
            if index + 1 == tup.1 {
                continue;
            }
        }
        let d = (index + 1).min(rowvec.len() - (index + 1));
        if index < rmid && (0..d).all(|i| rowvec[i] == rowvec[index + d - i])
            || index >= rmid && (0..d).all(|i| rowvec[index - i] == rowvec[index + i + 1])
        {
            h = index + 1;
        }
    }

    let mut v = 0;
    for index in col_fold_candidates {
        if let Some(tup) = p1_solution {
            // skip part1 solution
            if index + 1 == tup.0 {
                continue;
            }
        }
        let d = (index + 1).min(colvec.len() - (index + 1));
        if index < cmid && (0..d).all(|i| colvec[i] == colvec[index + d - i])
            || index >= cmid && (0..d).all(|i| colvec[index - i] == colvec[index + i + 1])
        {
            v = index + 1;
        }
    }

    (v, h)
}

fn get_grids(puzzle_lines: &[String]) -> Result<Vec<Array2<char>>, Box<dyn Error>> {
    let mut grids = vec![];
    let mut v = vec![];
    for line in puzzle_lines {
        if line.is_empty() {
            grids.push(get_grid(&v)?);
            v.clear();
        } else {
            v.push(line.to_string());
        }
    }
    grids.push(get_grid(&v)?);
    Ok(grids)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(get_grids(puzzle_lines)?
        .iter()
        .map(|g| reflection_indices(g, None))
        .map(|tup| tup.0 + 100 * tup.1)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let grids = get_grids(puzzle_lines)?;
    let mut total = 0;
    for g in &grids {
        let p1_solution = reflection_indices(g, None);
        for ((i, j), c) in g.indexed_iter() {
            let mut g2 = g.clone();
            g2[[i,j]] = match *c == '.' {
                true => '#',
                false => '.',
            };
            let tup = reflection_indices(&g2, Some(p1_solution));
            if tup == (0, 0) {
                continue;
            }
            total += tup.0 + 100 * tup.1;
            break;
        }
    }
    Ok(total)
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
        assert_eq!(part1(&puzzle_lines)?, 405);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 33780);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 400);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 23479);
        Ok(())
    }
}
