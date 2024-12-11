use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(
        data.iter().filter(|line| !line.is_empty()).map(|line| line.chars()),
    )?)
}

fn count_xmas(v: &[char]) -> usize {
    v.windows(4)
        .filter(|w| *w == ['X', 'M', 'A', 'S'] || *w == ['S', 'A', 'M', 'X'])
        .count()
}

fn part1(grid: &Matrix<char>) -> Result<usize, Box<dyn Error>> {
    let mut xmas_count = 0;

    // rows
    xmas_count += grid.iter().map(count_xmas).sum::<usize>();

    // columns
    xmas_count += grid.transposed().iter().map(count_xmas).sum::<usize>();

    // diagonals
    for cw in 0..4 {
        // process the 4 rotated matrix
        let rgrid = grid.rotated_cw(cw);

        for i in 3..rgrid.rows {
            // avoid processing duplicate diagonals
            if i == rgrid.rows - 1 && cw > 1 {
                continue;
            }

            // from a starting row in the first column walk north-east
            let start = (i, 0);
            let mut diag = vec![*rgrid.get(start).unwrap()];
            diag.extend(
                rgrid
                    .in_direction(start, directions::NE)
                    .map(|p| rgrid.get(p).unwrap())
                    .collect::<Vec<_>>(),
            );
            xmas_count += count_xmas(&diag);
        }
    }

    Ok(xmas_count)
}

fn part2(grid: &Matrix<char>) -> Result<usize, Box<dyn Error>> {
    let mut x_mas_count = 0;

    // walks the input matrix checking 3x3 squares for diagonals containing "mas"
    //        M.S
    //        .A.
    //        M.S

    for i in 0..grid.rows - 2 {
        for j in 0..grid.columns - 2 {
            // helper to test a diagonal
            let is_mas = |diag: &[char; 2]| -> bool { *diag == ['M', 'S'] || *diag == ['S', 'M'] };

            // middle char must be an 'A'
            if *grid.get((i + 1, j + 1)).unwrap() == 'A'
                // diagonal corners: top left, bottom right
                && is_mas(&[
                    *grid.get((i, j)).unwrap(),
                    *grid.get((i + 2, j + 2)).unwrap(),
                ])
                // diagonal corners: bottom left, top right
                && is_mas(&[
                    *grid.get((i + 2, j)).unwrap(),
                    *grid.get((i, j + 2)).unwrap(),
                ])
            {
                x_mas_count += 1;
            }
        }
    }
    Ok(x_mas_count)
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

    let grid = get_grid(&puzzle_lines)?;

    let n = part1(&grid)?;
    writeln!(stdout, "Answer Part 1 = {n}")?;
    let n = part2(&grid)?;
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
        let grid = get_grid(&puzzle_lines)?;
        assert_eq!(part1(&grid)?, 18);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        let grid = get_grid(&puzzle_lines)?;
        assert_eq!(part1(&grid)?, 2578);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        let grid = get_grid(&puzzle_lines)?;
        assert_eq!(part2(&grid)?, 9);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        let grid = get_grid(&puzzle_lines)?;
        assert_eq!(part2(&grid)?, 1972);
        Ok(())
    }
}
