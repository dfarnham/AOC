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

fn gmove(
    coord: (usize, usize),
    grid: &Array2<char>,
    faces_origin: &[(usize, usize)],
    direction: usize,
) -> Option<(usize, usize, usize)> {
    let (row, col) = match direction {
        0 => (coord.0 + 0, coord.1 + 1), // right
        1 => (coord.0 + 1, coord.1 + 0), // down
        2 => (coord.0 + 0, coord.1 - 1), // left
        3 => (coord.0 - 1, coord.1 + 0), // up
        _ => panic!("oops"),
    };

    let params = match direction {
        0 => (row as i64, 0, 1, 1),                        // right c
        1 => (1, 1, col as i64, 0),                        // down r
        2 => (row as i64, 0, grid.ncols() as i64 - 1, -1), // left c
        3 => (grid.nrows() as i64 - 1, -1, col as i64, 0), // up r
        _ => panic!("oops"),
    };

    match grid[[row, col]] {
        '.' => Some((row, col, direction)),
        ' ' => {
            let mut r = params.0;
            let mut c = params.2;
            while grid[[r as usize, c as usize]] == ' ' {
                r += params.1;
                c += params.3
            }
            match grid[[r as usize, c as usize]] {
                '.' => Some((r as usize, c as usize, direction)),
                _ => None,
            }
        }
        '#' => None,
        _ => panic!("oops"),
    }
}

fn left(coord: (usize, usize), grid: &Array2<char>) -> Option<(usize, usize)> {
    let (row, col) = coord;
    match grid[[row, col - 1]] {
        '.' => Some((row, col - 1)),
        ' ' => {
            let mut c = grid.ncols() - 1;
            while grid[[row, c]] == ' ' {
                c -= 1;
            }
            match grid[[row, c]] {
                '.' => Some((row, c)),
                _ => None,
            }
        }
        '#' => None,
        _ => panic!("oops"),
    }
}

fn right(coord: (usize, usize), grid: &Array2<char>) -> Option<(usize, usize)> {
    let (row, col) = coord;
    match grid[[row, col + 1]] {
        '.' => Some((row, col + 1)),
        ' ' => {
            let mut c = 1;
            while grid[[row, c]] == ' ' {
                c += 1;
            }
            match grid[[row, c]] {
                '.' => Some((row, c)),
                _ => None,
            }
        }
        '#' => None,
        _ => panic!("oops"),
    }
}

fn up(coord: (usize, usize), grid: &Array2<char>) -> Option<(usize, usize)> {
    let (row, col) = coord;
    match grid[[row - 1, col]] {
        '.' => Some((row - 1, col)),
        ' ' => {
            let mut r = grid.nrows() - 1;
            while grid[[r, col]] == ' ' {
                r -= 1;
            }
            match grid[[r, col]] {
                '.' => Some((r, col)),
                _ => None,
            }
        }
        '#' => None,
        _ => panic!("oops"),
    }
}

fn down(coord: (usize, usize), grid: &Array2<char>) -> Option<(usize, usize)> {
    let (row, col) = coord;
    match grid[[row + 1, col]] {
        '.' => Some((row + 1, col)),
        ' ' => {
            let mut r = 1;
            while grid[[r, col]] == ' ' {
                r += 1;
            }
            match grid[[r, col]] {
                '.' => Some((r, col)),
                _ => None,
            }
        }
        '#' => None,
        _ => panic!("oops"),
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (steps, directions, grid) = get_data(puzzle_lines);
    //println!("{:?}", directions);

    // find start coord
    let mut pos = (1, 1);
    for j in 1..grid.ncols() {
        if grid[[1, j]] == ' ' {
            continue;
        }
        pos.1 = j;
        break;
    }
    //println!("{:?}", pos);

    let mut sc = 0;
    let mut dc = 0;
    let mut direction = 0;
    loop {
        for _ in 0..steps[sc] {
            //println!("{z} go {direction} from {:?}", pos);

            if let Some(coord) = match direction {
                0 => right(pos, &grid),
                1 => down(pos, &grid),
                2 => left(pos, &grid),
                3 => up(pos, &grid),
                _ => panic!("oops"),
            } {
                pos = coord;
                //println!("  new pos {:?}", pos);
            } else {
                //println!("no move");
                break;
            }
        }
        sc += 1;

        if dc == directions.len() {
            break;
        }
        direction = match directions[dc] {
            'R' => (direction + 1) % 4,
            'L' => (direction + 4 - 1) % 4,
            _ => panic!("oops"),
        };
        dc += 1;
    }
    //println!("{:?}", pos);
    Ok(1000 * pos.0 + 4 * pos.1 + direction)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
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

    let mut faces_origin = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut sc = 0;
    let mut dc = 0;
    let mut direction = 0;
    loop {
        for _ in 0..steps[sc] {
            if let Some((r, c, d)) = gmove(pos, &grid, &faces_origin, direction) {
                pos = (r, c);
                direction = d;
            } else {
                //println!("no move");
                break;
            }
        }
        sc += 1;

        if dc == directions.len() {
            break;
        }
        direction = match directions[dc] {
            'R' => (direction + 1) % 4,
            'L' => (direction + 4 - 1) % 4,
            _ => panic!("oops"),
        };
        dc += 1;
    }
    //println!("{:?}", pos);
    Ok(1000 * pos.0 + 4 * pos.1 + direction)
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

    /*
    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 5031);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 3343167719435);
        Ok(())
    }
    */
}
