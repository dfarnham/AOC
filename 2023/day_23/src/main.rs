use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use pathfinding::matrix::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

type Point = (usize, usize);

fn get_grid(data: &[String]) -> Result<Matrix<char>, Box<dyn Error>> {
    Ok(Matrix::from_rows(data.iter().map(|line| line.chars()))?)
}

fn neighbors(m: &Matrix<char>, p: Point, p2: bool) -> Vec<Point> {
    let (i, j) = (p.0 as i32, p.1 as i32);
    let mut indices = vec![];

    // part1: (^, >, v, and <)
    if !p2 && m[(i as usize, j as usize)] == '^' {
        indices.push((i - 1, j));
    } else if !p2 && m[(i as usize, j as usize)] == '>' {
        indices.push((i, j + 1));
    } else if !p2 && m[(i as usize, j as usize)] == '<' {
        indices.push((i, j - 1));
    } else if !p2 && m[(i as usize, j as usize)] == 'v' {
        indices.push((i + 1, j));
    } else {
        // above
        indices.push((i - 1, j));

        // left
        indices.push((i, j - 1));

        // below
        indices.push((i + 1, j));

        // right
        indices.push((i, j + 1));
    }

    let mut ind = vec![];
    for e in indices {
        let (i, j) = e;
        if i < 0 || i >= m.rows as i32 || j < 0 || j >= m.columns as i32 || m[(i as usize, j as usize)] == '#' {
            continue;
        }
        ind.push((e.0 as usize, e.1 as usize))
    }
    ind
}

fn dfs(
    graph: &HashMap<Point, HashMap<Point, usize>>,
    seen: &mut HashSet<Point>,
    pt: Point,
    end: Point,
) -> Option<usize> {
    if pt == end {
        return Some(0);
    }
    let mut m = None;
    seen.insert(pt);
    for nx in graph[&pt].keys() {
        if !seen.contains(nx) {
            if let Some(d) = dfs(graph, seen, *nx, end) {
                m = m.max(Some(d + graph[&pt][nx]))
            }
        }
    }
    seen.remove(&pt);
    m
}

fn solution(grid: &Matrix<char>, p2: bool) -> usize {
    // find the start and end positions (localte the first '.' in the top and bottom row of the grid)
    let s = (
        0,
        grid.iter()
            .take(1)
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .position(|c| **c == '.')
            .unwrap(),
    );
    let e = (
        grid.rows - 1,
        grid.iter()
            .skip(grid.rows - 1)
            .take(1)
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .position(|c| **c == '.')
            .unwrap(),
    );
    //println!("s = {s:?}, e = {e:?}");

    let mut points = vec![s, e];
    for (r, row) in grid.iter().enumerate() {
        let verts = row
            .iter()
            .enumerate()
            .filter(|(_, ch)| **ch != '#')
            .filter(|(c, _)| neighbors(grid, (r, *c), p2).len() > 2)
            .map(|(c, _)| (r, c))
            .collect::<Vec<_>>();
        points.extend(verts);
    }

    let mut graph = HashMap::new();
    for pt in &points {
        graph.insert(*pt, HashMap::<Point, usize>::new());
    }
    //println!("{points:?}");

    for (sr, sc) in &points {
        let mut stack = vec![(0, *sr, *sc)];
        stack.push((0, *sr, *sc));
        let mut seen = HashSet::new();
        seen.insert((*sr, *sc));

        while let Some((n, r, c)) = stack.pop() {
            if n > 0 && graph.contains_key(&(r, c)) {
                let hash = graph.get_mut(&(*sr, *sc)).unwrap();
                hash.insert((r, c), n);
                continue;
            }
            for (nr, nc) in neighbors(grid, (r, c), p2) {
                if !seen.contains(&(nr, nc)) {
                    stack.push((n + 1, nr, nc));
                    seen.insert((nr, nc));
                }
            }
        }
    }
    //println!("graph = {graph:?}");

    dfs(&graph, &mut HashSet::new(), s, e).unwrap()
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
    writeln!(stdout, "Answer Part 1 = {}", solution(&grid, false))?;
    writeln!(stdout, "Answer Part 2 = {}", solution(&grid, true))?;

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
        assert_eq!(solution(&grid, false), 94);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid(&data)?;
        assert_eq!(solution(&grid, false), 2070);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example")?;
        let grid = get_grid(&data)?;
        assert_eq!(solution(&grid, true), 154);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual")?;
        let grid = get_grid(&data)?;
        assert_eq!(solution(&grid, true), 6498);
        Ok(())
    }
}
