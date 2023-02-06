use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, Write};

fn get_heatmap(data: &[String]) -> Array2<u32> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut heatmap = Array::zeros((0, data[0].len()));

    // process data[..]
    for line in data {
        heatmap.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    heatmap
}

fn get_lowpoints(heatmap: &Array2<u32>) -> Vec<(usize, usize)> {
    let (nrow, ncol) = (heatmap.nrows(), heatmap.ncols());

    let is_lowpoint = |r, c, n| {
        (r == 0 || heatmap[[r - 1, c]] > n)
            && (r + 1 == nrow || heatmap[[r + 1, c]] > n)
            && (c == 0 || heatmap[[r, c - 1]] > n)
            && (c + 1 == ncol || heatmap[[r, c + 1]] > n)
    };

    let mut lowpoints = vec![];
    for row in 0..nrow {
        for col in 0..ncol {
            if is_lowpoint(row, col, heatmap[[row, col]]) {
                lowpoints.push((row, col));
            }
        }
    }
    lowpoints
}

fn find_basin(heatmap: &Array2<u32>, point: &(usize, usize), basin: &mut BTreeSet<(usize, usize)>) {
    if basin.contains(point) {
        return;
    }
    basin.insert(*point);

    let (r, c) = *point;
    let n = heatmap[[r, c]];

    let mut adjacents = vec![];
    if r != 0 && heatmap[[r - 1, c]] > n {
        adjacents.push((r - 1, c));
    }
    if r + 1 < heatmap.nrows() && heatmap[[r + 1, c]] > n {
        adjacents.push((r + 1, c));
    }
    if c != 0 && heatmap[[r, c - 1]] > n {
        adjacents.push((r, c - 1));
    }
    if c + 1 < heatmap.ncols() && heatmap[[r, c + 1]] > n {
        adjacents.push((r, c + 1));
    }

    adjacents
        .iter()
        .filter(|(r, c)| heatmap[[*r, *c]] != 9)
        .for_each(|p| find_basin(heatmap, p, basin));
}

fn get_solution1(heatmap: &Array2<u32>) -> u32 {
    get_lowpoints(heatmap)
        .iter()
        .map(|(r, c)| heatmap[[*r, *c]] + 1)
        .sum::<_>()
}

fn get_solution2(heatmap: &Array2<u32>) -> u32 {
    let mut basin_sizes = vec![];

    for point in get_lowpoints(heatmap) {
        let mut basin = BTreeSet::<(usize, usize)>::new();
        find_basin(heatmap, &point, &mut basin);
        basin_sizes.push(basin.len());
    }

    assert!(basin_sizes.len() > 2);
    basin_sizes.sort_by(|a, b| b.cmp(a));
    //(basin_sizes[0] * basin_sizes[1] * basin_sizes[2]) as u32
    //basin_sizes.iter().take(3).fold(1, |acc, x| acc * x) as u32
    basin_sizes.iter().take(3).product::<usize>() as u32
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

    let heatmap = get_heatmap(&puzzle_lines);
    //println!("heatmap = {:?}", heatmap);

    writeln!(stdout, "Answer Part 1 = {}", get_solution1(&heatmap))?;
    writeln!(stdout, "Answer Part 2 = {}", get_solution2(&heatmap))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datapoints(filename: &str) -> Array2<u32> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        get_heatmap(&data)
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        assert_eq!(get_solution1(&data), 15);
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        assert_eq!(get_solution1(&data), 572);
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        assert_eq!(get_solution2(&data), 1134);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        assert_eq!(get_solution2(&data), 847044);
    }
}
