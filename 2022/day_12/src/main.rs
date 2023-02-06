use general::{get_args, read_data_lines, reset_sigpipe};
use ndarray::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

// parse the input to generate:
//   1. a 2-d array of unsigned integers
//   2. the `start` and `end` coordinates
//
// data types:
//   1. the matrix is an ndarray `Array2`
//   2. the `start` and `end` coordinates are tuples `(usize, usize)`
//
// the puzzle is to connect `start` to `end` by transitions (up, down, left, right)
// and find the shortest path
//
// a transition is legal unless the destination (up, down, left right) value
// is greater than the current value by more than 1
//
// the story treats it as hill climbing
//
fn parse_input(data: &[String]) -> (Array2<usize>, (usize, usize), (usize, usize)) {
    // row parsing rules for lines in data
    //
    // puzzle input represents a matrix of heights by lowercase characters
    // `start` is 'S' with value 'a'
    // `end` is 'E' with value 'z'
    // Example:
    //     Sabqponm
    //     abcryxxl
    //     accszExk
    //     acctuvwj
    //     abdefghi
    //
    // Reference
    //   ['A', 'Z] == [65, 90]
    //   ['a', 'z'] == [97, 122]
    //

    // use data[0] to size the new Array2
    assert!(data[0].len() > 1);
    let mut mat = Array::from_elem((0, data[0].len()), 0);

    // process data[..]
    let get_row = |s: &str| s.chars().map(|c| (c as usize)).collect::<Vec<_>>();
    for line in data {
        mat.push_row(ArrayView::from(&get_row(line))).unwrap()
    }

    // record start and reset 'S' to 'a'
    let (i, j) = mat
        .indexed_iter()
        .find(|(_, v)| *v == &('S' as usize))
        .expect("to find 'S'")
        .0;
    let start = (i, j);
    mat[[i, j]] = 'a' as usize;

    // record end and reset 'E' to 'z'
    let (i, j) = mat
        .indexed_iter()
        .find(|(_, v)| *v == &('E' as usize))
        .expect("to find 'E'")
        .0;
    let end = (i, j);
    mat[[i, j]] = 'z' as usize;

    (mat, start, end)
}

// neighbors matching transition contraint
//
// a transition is legal unless the neighbor (up, down, left right) value
// is greater than the position value by more than 1
//
// a list of legal indices is returned
fn neighbors(m: &Array2<usize>, p: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = p;
    let maxval = m[[i, j]] + 1;
    let mut indices = vec![];

    // above
    if i > 0 && m[[i - 1, j]] <= maxval {
        indices.push((i - 1, j))
    }

    // left
    if j > 0 && m[[i, j - 1]] <= maxval {
        indices.push((i, j - 1))
    }

    // below
    if i < m.nrows() - 1 && m[[i + 1, j]] <= maxval {
        indices.push((i + 1, j))
    }

    // right
    if j < m.ncols() - 1 && m[[i, j + 1]] <= maxval {
        indices.push((i, j + 1))
    }

    indices
}

fn solve(m: &Array2<usize>, s: (usize, usize), e: (usize, usize), part: usize) -> usize {
    // m = matrix of heights
    // s = starting coordinate
    // e = ending coordinate goal
    let mut visited = HashSet::<(usize, usize)>::new();

    // initialize the work queue for bfs with starting position(s) set to 0
    let mut q = VecDeque::new();
    if part == 1 {
        q.push_back((s, 0))
    } else {
        let start_value = m[[s.0, s.1]];

        // any/all coordinates matching the start_value
        for (coord, _) in m.indexed_iter().filter(|(_, v)| *v == &start_value) {
            q.push_back((coord, 0))
        }
    }

    while let Some((p, d)) = q.pop_front() {
        if !visited.contains(&p) {
            visited.insert(p);

            // when found return the distance
            if p == e {
                return d;
            }

            // add neighbors to the work queue, adding +1 to their distance
            for coord in neighbors(m, p).iter().copied() {
                q.push_back((coord, d + 1))
            }
        }
    }
    panic!("no solution")
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mat, s, e) = parse_input(puzzle_lines);
    Ok(solve(&mat, s, e, 1))
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mat, s, e) = parse_input(puzzle_lines);
    Ok(solve(&mat, s, e, 2))
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

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
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 31);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 350);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 29);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 349);
        Ok(())
    }
}
