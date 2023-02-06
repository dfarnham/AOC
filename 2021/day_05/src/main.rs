use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2};
use std::error::Error;
use std::io::{self, Write};

#[derive(Copy, Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Copy, Clone, Debug)]
struct LineSegment {
    p1: Point<u32>,
    p2: Point<u32>,
}

fn get_line_segments(data: &[String]) -> Vec<LineSegment> {
    let mut line_segments = vec![];
    for line in data {
        let coordinates = line
            .replace("->", ",")
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        line_segments.push(LineSegment {
            p1: Point {
                x: coordinates[0],
                y: coordinates[1],
            },
            p2: Point {
                x: coordinates[2],
                y: coordinates[3],
            },
        });
    }
    line_segments
}

fn get_grid_dimensions(segments: &[LineSegment]) -> (usize, usize) {
    let xmax = segments
        .iter()
        .map(|seg| seg.p1.x.max(seg.p2.x))
        .max()
        .expect("x max() failure");
    let ymax = segments
        .iter()
        .map(|seg| seg.p1.y.max(seg.p2.y))
        .max()
        .expect("y max() failure");
    (1 + xmax as usize, 1 + ymax as usize)
}

fn get_horizontal(segments: &[LineSegment]) -> Vec<LineSegment> {
    segments
        .iter()
        .filter(|seg| seg.p1.y == seg.p2.y)
        .copied()
        .collect::<Vec<_>>()
}

fn get_vertical(segments: &[LineSegment]) -> Vec<LineSegment> {
    segments
        .iter()
        .filter(|seg| seg.p1.x == seg.p2.x)
        .copied()
        .collect::<Vec<_>>()
}

fn get_diagonal(segments: &[LineSegment]) -> Vec<LineSegment> {
    segments
        .iter()
        .filter(|seg| seg.p1.x != seg.p2.x && seg.p1.y != seg.p2.y)
        .copied()
        .collect::<Vec<_>>()
}

fn update_grid_horiz_vert_count(segments: &[LineSegment], grid: &mut Array2<u32>) -> usize {
    for seg in get_horizontal(segments) {
        for x in seg.p1.x.min(seg.p2.x)..=seg.p1.x.max(seg.p2.x) {
            grid[[x as usize, seg.p1.y as usize]] += 1;
        }
    }
    for seg in get_vertical(segments) {
        for y in seg.p1.y.min(seg.p2.y)..=seg.p1.y.max(seg.p2.y) {
            grid[[seg.p1.x as usize, y as usize]] += 1;
        }
    }
    grid.iter().filter(|n| *n > &1).count()
}

fn update_grid_diag_count(segments: &[LineSegment], grid: &mut Array2<u32>) -> usize {
    for seg in get_diagonal(segments) {
        let mut x = seg.p1.x;
        let mut y = seg.p1.y;

        grid[[x as usize, y as usize]] += 1;
        // can test either x or y as the diagonal is traversed
        while x != seg.p2.x {
            x = match seg.p1.x < seg.p2.x {
                true => x + 1,
                false => x - 1,
            };
            y = match seg.p1.y < seg.p2.y {
                true => y + 1,
                false => y - 1,
            };
            grid[[x as usize, y as usize]] += 1;
        }
    }
    grid.iter().filter(|n| *n > &1).count()
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

    let segments = get_line_segments(&puzzle_lines);
    let mut grid = Array::from_elem(get_grid_dimensions(&segments), 0);
    let horiz_vert_overlap_count = update_grid_horiz_vert_count(&segments, &mut grid);
    let horiz_vert_diag_overlap_count = update_grid_diag_count(&segments, &mut grid);

    writeln!(stdout, "Answer Part 1 = {horiz_vert_overlap_count}")?;
    writeln!(stdout, "Answer Part 2 = {horiz_vert_diag_overlap_count}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1(filename: &str) -> usize {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        let segments = get_line_segments(&data);
        let mut grid = Array::from_elem(get_grid_dimensions(&segments), 0);
        update_grid_horiz_vert_count(&segments, &mut grid)
    }

    fn part2(filename: &str) -> usize {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        let segments = get_line_segments(&data);
        let mut grid = Array::from_elem(get_grid_dimensions(&segments), 0);
        update_grid_horiz_vert_count(&segments, &mut grid);
        update_grid_diag_count(&segments, &mut grid)
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("input-example"), 5);
    }

    #[test]
    fn part1_actual() {
        assert_eq!(part1("input-actual"), 5092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("input-example"), 12);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2("input-actual"), 20484);
    }
}
