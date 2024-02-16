use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

//
// This ended up being a playground for drawing the pipes
//

const PIPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];
const UP: (i64, i64) = (-1, 0);
const DOWN: (i64, i64) = (1, 0);
const LEFT: (i64, i64) = (0, -1);
const RIGHT: (i64, i64) = (0, 1);

/// Pipe
/// postion: (i,j)
/// c: piece
///
/// methods
///   adjacents(): surrounding coordinates within the grid that touch the number
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pipe {
    i: usize, // i coordinate
    j: usize, // j coordinate
    c: char,
}
impl Pipe {
    fn is_s(&self) -> bool {
        self.c == 'S'
    }

    fn adjacents(&self, grid: &Array2<Pipe>) -> HashSet<(usize, usize)> {
        let (nrow, ncol) = grid.dim();
        let mut coords = HashSet::new();
        for (x, y) in [UP, DOWN, LEFT, RIGHT] {
            let i = x + self.i as i64;
            let j = y + self.j as i64;
            if i >= 0 && j >= 0 && i < nrow as i64 && j < ncol as i64 {
                let i = i as usize;
                let j = j as usize;
                if self.connects(&grid[[i, j]]) {
                    coords.insert((i, j));
                }
            }
        }
        coords
    }

    // does self connect to pipe?
    fn connects(&self, pipe: &Pipe) -> bool {
        let rowdiff = (self.i as i64 - pipe.i as i64).abs() == 1;
        let coldiff = (self.j as i64 - pipe.j as i64).abs() == 1;
        if pipe.c == 'S' {
            return true;
        }
        if self.c == 'S' {
            return true;
        }
        if !"|-LJ7F".contains(pipe.c) || !(rowdiff || coldiff) {
            return false;
        }

        let samerow = !rowdiff && coldiff && (self.i == pipe.i);
        let samecol = !coldiff && rowdiff && (self.j == pipe.j);
        //println!("{self:?} against {pipe:?}; rowdiff={rowdiff}, coldiff={coldiff}, samerow={samerow}, samecol={samecol}");
        let up = samecol && pipe.i < self.i;
        let down = samecol && pipe.i > self.i;
        let left = samerow && pipe.j < self.j;
        let right = samerow && pipe.j > self.j;
        //println!("{self:?} against {pipe:?} up={up}, down={down}, left={left}, right={right}, samerow={samerow}, samecol={samecol}");
        match self.c {
            '|' if up && "|7F".contains(pipe.c) => true,
            'L' if up && "|7F".contains(pipe.c) => true,
            'J' if up && "|7F".contains(pipe.c) => true,
            '|' if down && "|LJ".contains(pipe.c) => true,
            '7' if down && "|LJ".contains(pipe.c) => true,
            'F' if down && "|LJ".contains(pipe.c) => true,
            '-' if left && "-LF".contains(pipe.c) => true,
            'J' if left && "-LF".contains(pipe.c) => true,
            '7' if left && "-LF".contains(pipe.c) => true,
            '-' if right && "-J7".contains(pipe.c) => true,
            'L' if right && "-J7".contains(pipe.c) => true,
            'F' if right && "-J7".contains(pipe.c) => true,
            _ => false,
        }
    }
    fn direction(&self, pipe: Pipe) -> (bool, bool, bool, bool) {
        let rowdiff = (self.i as i64 - pipe.i as i64).abs() == 1;
        let coldiff = (self.j as i64 - pipe.j as i64).abs() == 1;

        let samerow = !rowdiff && coldiff && (self.i == pipe.i);
        let samecol = !coldiff && rowdiff && (self.j == pipe.j);
        //println!("{self:?} against {pipe:?}; rowdiff={rowdiff}, coldiff={coldiff}, samerow={samerow}, samecol={samecol}");
        let up = samecol && pipe.i < self.i;
        let down = samecol && pipe.i > self.i;
        let left = samerow && pipe.j < self.j;
        let right = samerow && pipe.j > self.j;
        (up, down, left, right)
    }

    fn graphic(&self, pipe: &Pipe) -> char {
        let rowdiff = (self.i as i64 - pipe.i as i64).abs() == 1;
        let coldiff = (self.j as i64 - pipe.j as i64).abs() == 1;

        let samerow = !rowdiff && coldiff && (self.i == pipe.i);
        let samecol = !coldiff && rowdiff && (self.j == pipe.j);
        //println!("{self:?} against {pipe:?}; rowdiff={rowdiff}, coldiff={coldiff}, samerow={samerow}, samecol={samecol}");
        let up = samecol && pipe.i < self.i;
        let down = samecol && pipe.i > self.i;
        let left = samerow && pipe.j < self.j;
        let right = samerow && pipe.j > self.j;
        //println!("{self:?} against {pipe:?} up={up}, down={down}, left={left}, right={right}, samerow={samerow}, samecol={samecol}");
        // '\u{2190}' left ←
        // '\u{2192}' right →
        // '\u{2191}' up ↑
        // '\u{2193}' down ↓
        // '\u{21B0}' up_left ↰
        // '\u{21B1}' up_right ↱
        // '\u{21B2}' down_left ↲
        // '\u{21B3}' down_right ↳

        //$ echo -e "↱→⬎\n↑.↓\n⬑←↲"
        // ↱→⬎
        // ↑.↓
        // ⬑←↲

        //$ echo -e "⬐←↰\n↓.↑\n↳→⬏"
        // ⬐←↰
        // ↓.↑
        // ↳→⬏
        match self.c {
            '|' if up && "|".contains(pipe.c) => '↑',
            '|' if up && "7".contains(pipe.c) => '↰',
            '|' if up && "F".contains(pipe.c) => '↱',
            'L' if up && "|".contains(pipe.c) => '↑',
            'L' if up && "7".contains(pipe.c) => '↰',
            'L' if up && "F".contains(pipe.c) => '↱',
            'J' if up && "|".contains(pipe.c) => '↑',
            'J' if up && "7".contains(pipe.c) => '↰',
            'J' if up && "F".contains(pipe.c) => '↱',
            '|' if down && "|".contains(pipe.c) => '↓',
            '|' if down && "L".contains(pipe.c) => '↳',
            '|' if down && "J".contains(pipe.c) => '↲',
            '7' if down && "|".contains(pipe.c) => '↓',
            '7' if down && "L".contains(pipe.c) => '↳',
            '7' if down && "J".contains(pipe.c) => '↲',
            'F' if down && "|".contains(pipe.c) => '↓',
            'F' if down && "L".contains(pipe.c) => '↳',
            'F' if down && "J".contains(pipe.c) => '↲',
            '-' if left && "-".contains(pipe.c) => '←',
            '-' if left && "L".contains(pipe.c) => '⬑',
            '-' if left && "F".contains(pipe.c) => '⬐',
            'J' if left && "-".contains(pipe.c) => '←',
            'J' if left && "L".contains(pipe.c) => '⬑',
            'J' if left && "F".contains(pipe.c) => '⬐',
            '7' if left && "-".contains(pipe.c) => '←',
            '7' if left && "L".contains(pipe.c) => '⬑',
            '7' if left && "F".contains(pipe.c) => '⬐',
            '-' if right && "-".contains(pipe.c) => '→',
            '-' if right && "J".contains(pipe.c) => '⬏',
            '-' if right && "7".contains(pipe.c) => '⬎',
            'L' if right && "-".contains(pipe.c) => '→',
            'L' if right && "J".contains(pipe.c) => '⬏',
            'L' if right && "7".contains(pipe.c) => '⬎',
            'F' if right && "-".contains(pipe.c) => '→',
            'F' if right && "J".contains(pipe.c) => '⬏',
            'F' if right && "7".contains(pipe.c) => '⬎',
            _ => unreachable!(),
        }
    }
}

fn get_grid(data: &[String]) -> Result<Array2<Pipe>, Box<dyn Error>> {
    // row parsing rules for lines in data
    let get_row = |s: &str| s.chars().collect::<Vec<_>>();

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), '.');

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }

    let pipe_grid: Vec<Pipe> = grid
        .indexed_iter()
        .map(|elem| Pipe {
            i: elem.0 .0,
            j: elem.0 .1,
            c: *elem.1,
        })
        .collect();
    Ok(Array::from_shape_vec(grid.dim(), pipe_grid)?)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut grid = get_grid(puzzle_lines)?;
    let s = *grid.iter().find(|pipe| pipe.is_s()).unwrap();
    //println!("s = {s:?}");

    let mut found = vec![];

    for schar in PIPES {
        let possible_s = Pipe {
            i: s.i,
            j: s.j,
            c: schar,
        };
        grid[[s.i, s.j]] = possible_s;

        for mut elem in grid
            .clone()
            .into_iter()
            .filter(|elem| elem.adjacents(&grid).contains(&(s.i, s.j)))
        {
            let mut prev = (s.i, s.j);
            let mut solutions = vec![];
            while let Some(point) = elem.adjacents(&grid).iter().find(|p| *p != &prev) {
                solutions.push(elem);
                prev = (elem.i, elem.j);
                elem = grid[[point.0, point.1]];
                if elem.adjacents(&grid).contains(&(s.i, s.j)) {
                    solutions.push(elem);
                    solutions.push(possible_s);
                    found.push(solutions.clone());
                    solutions.clear();
                    break;
                }
            }
        }
    }

    Ok(match found.clone().iter().map(|f| f.len()).max() {
        Some(max) => max / 2,
        _ => 0,
    })
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut grid = get_grid(puzzle_lines)?;
    let s = *grid.iter().find(|pipe| pipe.is_s()).unwrap();

    let mut found = vec![];

    for schar in PIPES {
        let possible_s = Pipe {
            i: s.i,
            j: s.j,
            c: schar,
        };
        grid[[s.i, s.j]] = possible_s;

        for mut elem in grid
            .clone()
            .into_iter()
            .filter(|elem| elem.adjacents(&grid).contains(&(s.i, s.j)))
        {
            let mut prev = (s.i, s.j);
            let mut solutions = vec![];
            while let Some(point) = elem.adjacents(&grid).iter().find(|p| *p != &prev) {
                solutions.push(elem);
                prev = (elem.i, elem.j);
                elem = grid[[point.0, point.1]];
                if elem.adjacents(&grid).contains(&(s.i, s.j)) {
                    solutions.push(elem);
                    solutions.push(possible_s);
                    found.push(solutions.clone());
                    solutions.clear();
                    break;
                }
            }
        }
    }

    let loop_pipes = &found[0];
    let mut chain = loop_pipes
        .windows(2)
        .map(|w| (w[0], w[0].direction(w[1])))
        .collect::<Vec<_>>();
    let lst = loop_pipes[loop_pipes.len() - 1];
    chain.push((lst, lst.direction(loop_pipes[0])));

    // returns a pipe in the proximity in direction (avoids loop points)
    let (nrow, ncol) = grid.dim();
    let proximity_pipe = |d: (i64, i64), i: usize, j: usize| -> Option<Pipe> {
        let ii = d.0 + i as i64;
        let jj = d.1 + j as i64;
        if ii >= 0 && jj >= 0 && ii < nrow as i64 && jj < ncol as i64 {
            let ii = ii as usize;
            let jj = jj as usize;
            if !loop_pipes.contains(&grid[[ii, jj]]) {
                return Some(grid[[ii, jj]]);
            }
        }
        None
    };

    let mut inside = HashSet::new();
    for chn in chain {
        let (pipe, (up, down, left, right)) = chn;
        let (i, j) = (pipe.i, pipe.j);
        match (up, down, left, right) {
            (true, false, false, false) => {
                // up
                if let Some(ipipe) = proximity_pipe(RIGHT, i, j) {
                    inside.insert(ipipe);
                }
            }
            (false, true, false, false) => {
                // down
                if let Some(ipipe) = proximity_pipe(LEFT, i, j) {
                    inside.insert(ipipe);
                }
            }
            (false, false, true, false) => {
                // left
                if let Some(ipipe) = proximity_pipe(UP, i, j) {
                    inside.insert(ipipe);
                }
            }
            (false, false, false, true) => {
                // right
                if let Some(ipipe) = proximity_pipe(DOWN, i, j) {
                    inside.insert(ipipe);
                }
            }
            _ => unreachable!(),
        }
    }

    //println!("inside = {inside:?}");
    let mut changed = true;
    while changed {
        changed = false;
        for ((i, j), p) in grid.indexed_iter() {
            if inside.contains(p) {
                for d in [UP, DOWN, LEFT, RIGHT] {
                    if let Some(ipipe) = proximity_pipe(d, i, j) {
                        if !inside.contains(&ipipe) {
                            inside.insert(ipipe);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    let mut outside = HashSet::new();
    for ((i, j), p) in grid.indexed_iter().filter(|tup| !loop_pipes.contains(tup.1)) {
        if i == 0 || i == nrow - 1 || j == 0 || j == ncol - 1 {
            outside.insert(*p);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for ((i, j), p) in grid.indexed_iter().filter(|t| !loop_pipes.contains(t.1)) {
            if outside.contains(p) {
                for d in [UP, DOWN, LEFT, RIGHT] {
                    if let Some(ipipe) = proximity_pipe(d, i, j) {
                        if !inside.contains(&ipipe) && !outside.contains(&ipipe) {
                            outside.insert(ipipe);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    for row in grid.rows() {
        for elem in row {
            if loop_pipes.contains(elem) {
                let mut blah = false;
                for w in loop_pipes.windows(2) {
                    if w[1] == *elem {
                        print!("{}", w[0].graphic(&w[1]));
                        blah = true;
                        break;
                    }
                }
                if !blah {
                    print!("•");
                }
            } else if outside.contains(elem) {
                print!("O");
            } else if inside.contains(elem) {
                print!("I");
            } else {
                //print!("{}", elem.c);
                print!("•");
            }
        }
        println!();
    }

    Ok(inside.len() + 1)
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
        assert_eq!(part1(&puzzle_lines)?, 4);
        Ok(())
    }

    /*
    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part1(&puzzle_lines)?, 8);
        Ok(())
    }
    */

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 6725);
        Ok(())
    }

    /*
    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part2(&puzzle_lines)?, 4);
        Ok(())
    }
    */

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 383);
        Ok(())
    }
}
