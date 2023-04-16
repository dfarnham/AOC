// this is so ugly having 2 different cube foldings, i don't know how to generalize it?
//
use general::{get_args, read_data_lines, reset_sigpipe};
use ndarray::*;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> (Vec<usize>, Vec<char>, Array2<char>) {
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    let m = data[0..data.len() - 2]
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap();
    let mut grid = Array::from_elem((0, m + 2), ' ');

    let mut border = String::new();
    for _ in 0..m + 2 {
        border.push(' '); // pad
    }

    // top border
    grid.push_row(ArrayView::from(&get_row(&border))).unwrap();

    // process data[..]
    for line in data {
        if line.is_empty() {
            break;
        }
        let mut maze = String::from(" "); // left edge
        maze += line;
        for _ in 0..m - line.len() {
            maze += " ";
        }
        maze += " "; // right edge
        grid.push_row(ArrayView::from(&get_row(&maze))).unwrap()
    }

    // bottom border
    grid.push_row(ArrayView::from(&get_row(&border))).unwrap();

    let nums_only = str::replace(&data[data.len() - 1], "R", " ");
    let nums_only = str::replace(&nums_only, "L", " ");
    let steps = nums_only
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let directions = data[data.len() - 1]
        .chars()
        .filter(|c| ['L', 'R'].contains(c))
        .collect::<Vec<_>>();
    (steps, directions, grid)
}

fn gmove1(coord: (usize, usize), grid: &Array2<char>, direction: usize) -> Option<(usize, usize)> {
    let (row, col) = match direction {
        0 => (coord.0, coord.1 + 1), // right
        1 => (coord.0 + 1, coord.1), // down
        2 => (coord.0, coord.1 - 1), // left
        3 => (coord.0 - 1, coord.1), // up
        _ => unreachable!(),
    };

    let params = match direction {
        0 => (row as i64, 0, 1, 1),                        // right c
        1 => (1, 1, col as i64, 0),                        // down r
        2 => (row as i64, 0, grid.ncols() as i64 - 1, -1), // left c
        3 => (grid.nrows() as i64 - 1, -1, col as i64, 0), // up r
        _ => unreachable!(),
    };

    match grid[[row, col]] {
        '.' => Some((row, col)),
        ' ' => {
            let mut r = params.0;
            let mut c = params.2;
            while grid[[r as usize, c as usize]] == ' ' {
                r += params.1;
                c += params.3
            }
            match grid[[r as usize, c as usize]] {
                '.' => Some((r as usize, c as usize)),
                _ => None,
            }
        }
        '#' => None,
        _ => unreachable!(),
    }
}

fn gmove2(
    coord: (usize, usize),
    grid: &Array2<char>,
    faces_origin: &[(usize, usize)],
    direction: usize,
    sz: usize,
) -> Option<(usize, usize, usize)> {
    let (row, col) = match direction {
        0 => (coord.0, coord.1 + 1), // right
        1 => (coord.0 + 1, coord.1), // down
        2 => (coord.0, coord.1 - 1), // left
        3 => (coord.0 - 1, coord.1), // up
        _ => unreachable!(),
    };

    let (row, col, direction) = if grid[[row, col]] == ' ' {
        transition(coord, direction, sz, faces_origin)
    } else {
        (row, col, direction)
    };

    match grid[[row, col]] {
        '.' => Some((row, col, direction)),
        _ => None,
    }
}

fn transition(
    src: (usize, usize),   // current position (r, c)
    direction: usize,      // (0=right, 1=down, 2=left, 3=up)
    sz: usize,             // face size (4, 50)
    fo: &[(usize, usize)], // faces origin
) -> (usize, usize, usize) {
    let sz = sz - 1;

    // what face is the input on?
    // return the index by checking the origins
    let mut face = None;
    for (i, corner) in fo.iter().enumerate() {
        if src.0 >= corner.0
            && src.0 <= corner.0 + sz
            && src.1 >= corner.1
            && src.1 <= corner.1 + sz
        {
            face = Some(i);
            break;
        }
    }
    let face = face.expect("face plant");

    // relative edge distances for output mapping
    let (dr, dc) = (src.0 - fo[face].0, src.1 - fo[face].1);
    let (rdr, rdc) = (sz - dr, sz - dc);

    // TODO: Notes to self.
    //   this is where i couldn't figure out a general solution to the shape
    //   of the input i'm processing, or i missed the boat.
    //
    //   given the enumerated face origins, position and size, along with an
    //   edge point and it's direction, it feels like i should be able to derive more.
    //
    //   So now this function is using a table populated with output faces and directions
    //
    //   1. i have the size of a face szXsz (puzzle example, actuals are 4,50)
    //   2. i have distances from the origin for the "aligned/not-aligned" mappings.
    //      "sz, dr, rdr" are used to map output rows
    //      "sz, dc, rdc" are used to map output columns
    //
    //   we are only called when the grid position matches a space ' '
    //      let (row, col, direction) = if grid[[row, col]] == ' ' {
    //          transition(coord, direction, sz, faces_origin)
    //      } else {
    //          (row, col, direction)
    //      };
    //   so this crashes on maps which aren't the same ascii shape
    //   i'm not sure how many different shapes can be formed unfolding a cube?
    //
    //   i eventually cut out shapes matching my input so i could hardcode the translation table
    //
    //
    // Hard Coding:
    // trans is an array of 6 elements (indexed by src face and hardcoded destination face)
    // trans[face] is an array of 4 arrays to be indexed by the input direction
    // the array elements info:
    //   [ destination face, row offset, column offset, destination direction ]
    //   [99, 99, 99, 99] represent unused directions for my input shapes
    let trans = match sz < 49 {
        true => [
            // indexed by direction (0=right, 1=down, 2=left, 3=up)
            [
                [5, 0, rdr, 2], // logically: input face 1 (right) maps to -> destination face (6, left) or [5, _, _, 2]
                [99, 99, 99, 99],
                [2, 0, dr, 1],  // 1 left -> 3 down
                [1, 0, rdc, 1], // 1 up -> 2 down
            ],
            [
                [99, 99, 99, 99],
                [4, sz, rdc, 3], // 2 down -> 5 up
                [5, sz, rdr, 3], // 2 left -> 6 up
                [0, 0, rdc, 1],  // 2 up -> 1 down
            ],
            [
                [99, 99, 99, 99],
                [4, rdc, 0, 0], // 3 down -> 5 right
                [99, 99, 99, 99],
                [0, dc, 0, 0], // 3 up -> 1 right
            ],
            [
                [5, 0, rdr, 1], //  4 right -> 6 down
                [99, 99, 99, 99],
                [99, 99, 99, 99],
                [99, 99, 99, 99],
            ],
            [
                [99, 99, 99, 99],
                [1, sz, rdc, 3], // 5 down -> 2 up
                [2, sz, rdr, 3], // 5 left -> 3 up
                [99, 99, 99, 99],
            ],
            [
                [0, rdr, sz, 2], // 6 right -> 1 left
                [1, rdc, 0, 0],  // 6 down -> 2 right
                [99, 99, 99, 99],
                [3, rdc, sz, 2], // 6 up -> 4 left
            ],
        ],
        false => [
            [
                [99, 99, 99, 99],
                [99, 99, 99, 99],
                [3, rdr, 0, 0], // 1 left  -> 4 right
                [5, dc, 0, 0],  // 1 up    -> 6 right
            ],
            [
                [4, rdr, sz, 2], // 2 right -> 5 left
                [2, dc, sz, 2],  // 2 down  -> 3 left
                [99, 99, 99, 99],
                [5, sz, dc, 3], // 2 up    -> 6 up
            ],
            [
                [1, sz, dr, 3], // 3 right -> 2 up
                [99, 99, 99, 99],
                [3, 0, dr, 1], // 3 left  -> 4 down
                [99, 99, 99, 99],
            ],
            [
                [99, 99, 99, 99],
                [99, 99, 99, 99],
                [0, rdr, 0, 0], //  4 left -> 1 right
                [2, dc, 0, 0],  //  4 up   -> 3 right
            ],
            [
                [1, rdr, sz, 2], // 5 right -> 2 left
                [5, dc, sz, 2],  // 5 down  -> 6 left
                [99, 99, 99, 99],
                [99, 99, 99, 99],
            ],
            [
                [4, sz, dr, 3], // 6 right -> 5 up
                [1, 0, dc, 1],  // 6 down  -> 2 down
                [0, 0, dr, 1],  // 6 left  -> 1 down
                [99, 99, 99, 99],
            ],
        ],
    };

    let info = trans[face][direction];
    let dst_face = info[0];
    assert!(dst_face < 6);
    (fo[dst_face].0 + info[1], fo[dst_face].1 + info[2], info[3])
}

fn solve(puzzle_lines: &[String], part: usize) -> Result<usize, Box<dyn Error>> {
    let (steps, directions, grid) = get_data(puzzle_lines);

    // find start coord
    let mut pos = (1, 1);
    for j in 1..grid.ncols() {
        if grid[[1, j]] == ' ' {
            continue;
        }
        pos.1 = j;
        break;
    }

    let mut faces_origin = vec![];
    let sz = (grid.nrows() - 2).max(grid.ncols() - 2) / 4;

    for i in (1..grid.nrows()).step_by(sz) {
        for j in (1..grid.ncols()).step_by(sz) {
            if grid[[i, j]] == ' ' {
                continue;
            }
            faces_origin.push((i, j));
        }
    }

    let mut sc = 0;
    let mut dc = 0;
    let mut direction = 0;
    loop {
        for _ in 0..steps[sc] {
            match part {
                1 => {
                    // grid moves wrapping around
                    if let Some(coord) = gmove1(pos, &grid, direction) {
                        pos = coord;
                    } else {
                        break;
                    }
                }
                2 => {
                    // grid moves walking the cube faces
                    if let Some((r, c, d)) = gmove2(pos, &grid, &faces_origin, direction, sz) {
                        pos = (r, c);
                        direction = d;
                    } else {
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
        sc += 1;

        if dc == directions.len() {
            break;
        }
        direction = match directions[dc] {
            'R' => (direction + 1) % 4,
            'L' => (direction + 4 - 1) % 4,
            _ => unreachable!(),
        };
        dc += 1;
    }

    Ok(1000 * pos.0 + 4 * pos.1 + direction)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 2)
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
        assert_eq!(part1(&puzzle_lines)?, 6032);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 26558);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 5031);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 110400);
        Ok(())
    }
}
