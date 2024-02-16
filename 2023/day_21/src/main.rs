use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(data.iter().map(|line| line.chars()))?)
}

#[allow(clippy::vec_init_then_push)]
fn neighbors(m: &Matrix<char>, p: (i32, i32)) -> Vec<(i32, i32)> {
    let (i, j) = p;
    let mut indices = vec![];

    // above
    indices.push((i - 1, j));

    // left
    indices.push((i, j - 1));

    // below
    indices.push((i + 1, j));

    // right
    indices.push((i, j + 1));

    let mut ind = vec![];
    for e in indices {
        let (i, j) = e;
        if i < 0 || i >= m.rows as i32 || j < 0 || j >= m.columns as i32 || m[(i as usize, j as usize)] == '#' {
            continue;
        }
        ind.push(e)
    }
    ind
}

// https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day21p2.py
fn fill(grid: &Matrix<char>, sr: usize, sc: usize, ss: usize) -> usize {
    let mut ans = HashSet::new();
    let mut seen = HashSet::new();
    let mut workq = VecDeque::new();
    seen.insert((sr as i32, sc as i32));
    workq.push_back((sr, sc, ss));
    while let Some((r, c, s)) = workq.pop_front() {
        if s % 2 == 0 {
            ans.insert((r, c));
        }
        if s == 0 {
            continue;
        }

        for (nr, nc) in neighbors(grid, (r as i32, c as i32)) {
            if seen.contains(&(nr, nc)) {
                continue;
            }
            seen.insert((nr, nc));
            workq.push_back((nr as usize, nc as usize, s - 1));
        }
    }
    ans.len()
}

fn part1(grid: &Matrix<char>, steps: usize) -> usize {
    // find the start position
    let mut s = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == 'S') {
            s = (i, j);
            break;
        }
    }

    let mut actual = HashSet::new();
    actual.insert((s.0 as i32, s.1 as i32));
    for _ in 0..steps {
        actual = HashSet::from_iter(
            actual
                .iter()
                .flat_map(|e| neighbors(grid, *e).into_iter())
                .collect::<Vec<_>>(),
        );
    }
    actual.len()
}

fn part2(grid: &Matrix<char>, steps: usize) -> usize {
    // find the start position
    let mut s = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == 'S') {
            s = (i, j);
            break;
        }
    }

    let size = grid.rows;
    let (sr, sc) = s;

    let grid_width = steps / size - 1;
    let odd = grid_width / 2 * 2 + 1;
    let odd = odd * odd;
    let even = (grid_width + 1) / 2 * 2;
    let even = even * even;

    let odd_points = fill(grid, sr, sc, size * 2 + 1);
    let even_points = fill(grid, sr, sc, size * 2);

    let corner_t = fill(grid, size - 1, sc, size - 1);
    let corner_r = fill(grid, sr, 0, size - 1);
    let corner_b = fill(grid, 0, sc, size - 1);
    let corner_l = fill(grid, sr, size - 1, size - 1);

    let small_tr = fill(grid, size - 1, 0, size / 2 - 1);
    let small_tl = fill(grid, size - 1, size - 1, size / 2 - 1);
    let small_br = fill(grid, 0, 0, size / 2 - 1);
    let small_bl = fill(grid, 0, size - 1, size / 2 - 1);

    let large_tr = fill(grid, size - 1, 0, size * 3 / 2 - 1);
    let large_tl = fill(grid, size - 1, size - 1, size * 3 / 2 - 1);
    let large_br = fill(grid, 0, 0, size * 3 / 2 - 1);
    let large_bl = fill(grid, 0, size - 1, size * 3 / 2 - 1);

    odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl)
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

    let grid = get_grid(&puzzle_lines)?;
    writeln!(stdout, "Answer Part 1 = {}", part1(&grid, 64))?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&grid, 26501365))?;

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
        Ok(read_trimmed_data_lines::<String>(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example")?;
        let grid = get_grid(&data)?;
        assert_eq!(part1(&grid, 6), 16);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid(&data)?;
        assert_eq!(part1(&grid, 64), 3671);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid(&data)?;
        assert_eq!(part2(&grid, 26501365), 609708004316870);
        Ok(())
    }
}
