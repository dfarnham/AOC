use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use ndarray::*;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

#[rustfmt::skip]
#[derive(Debug, PartialEq)]
struct Tile {
    mat: Array2<bool>,
}
impl Tile {
    fn edge_vector(&self, n: usize) -> Vec<bool> {
        match n {
            0 => self.mat.slice(s![.., 0]).to_vec(),
            1 => self.mat.slice(s![.., -1]).to_vec(),
            2 => self.mat.slice(s![0, ..]).to_vec(),
            3 => self.mat.slice(s![-1, ..]).to_vec(),
            _ => unreachable!(),
        }
    }

    fn inner_data(&self) -> Array2<bool> {
        let clippy_reversed_empty_ranges = -1;
        self.mat
            .slice(s![1..clippy_reversed_empty_ranges, 1..clippy_reversed_empty_ranges])
            .to_owned()
    }
}

fn get_tiles(data: &[String]) -> HashMap<usize, Tile> {
    let re: Regex = Regex::new(r"Tile (\d+):").unwrap();
    let mut rows = vec![];
    let mut ids = vec![];
    for line in data.iter().filter(|line| !line.is_empty()) {
        if line.starts_with("Tile") {
            let caps = re.captures(line).unwrap();
            let id = caps
                .get(1)
                .expect("capture failed")
                .as_str()
                .parse::<usize>()
                .expect("Can't parse");
            ids.push(id);
        } else {
            rows.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }

    // tiles are square: nXn
    let n = rows[0].len();

    let mut tiles = HashMap::new();

    for (i, id) in ids.iter().enumerate() {
        let mut mat = Array::from_elem((0, n), false);
        for row in rows.iter().skip(i * n).take(n) {
            mat.push_row(ArrayView::from(row)).unwrap();
        }
        tiles.insert(*id, Tile { mat });
    }
    tiles
}

fn rotate_flip(degrees: i32, horiz_flip: bool, vert_flip: bool, tile: &Array2<bool>) -> Array2<bool> {
    assert_eq!((degrees + 360) % 90, 0, "degrees = {degrees} is not a multiple of 90");
    let mut rotated_tile = tile.clone();
    let dim = tile.nrows();
    let rotation = (degrees + 360) / 90 % 4 * 90;

    if rotation > 0 {
        for i in 0..dim {
            for j in 0..dim {
                rotated_tile[[i, j]] = match (degrees + 360) / 90 % 4 * 90 {
                    90 => tile[[dim - 1 - j, i]],
                    180 => tile[[dim - 1 - i, dim - 1 - j]],
                    270 => tile[[j, dim - 1 - i]],
                    _ => tile[[i, j]], // NOTREACHED
                }
            }
        }
    }

    if horiz_flip {
        rotated_tile = flip_horizontal_axis(&rotated_tile);
    }

    if vert_flip {
        rotated_tile = flip_vertical_axis(&rotated_tile);
    }

    rotated_tile
}

fn flip_vertical_axis(tile: &Array2<bool>) -> Array2<bool> {
    rotate_flip(90, false, false, &tile.t().to_owned())
}

fn flip_horizontal_axis(tile: &Array2<bool>) -> Array2<bool> {
    rotate_flip(-90, false, false, &tile.t().to_owned())
}

#[allow(clippy::type_complexity)]
fn get_edges(tiles: &HashMap<usize, Tile>) -> HashMap<usize, (Vec<bool>, Vec<bool>, Vec<bool>, Vec<bool>)> {
    /*
    let mut edges = HashMap::new();

    for (id, tile) in tiles {
        edges.insert(
            *id,
            (
                tile.edge_vector(0),
                tile.edge_vector(1),
                tile.edge_vector(2),
                tile.edge_vector(3),
            ),
        );
    }
    edges
    */

    tiles
        .iter()
        .map(|(id, tile)| {
            (
                *id,
                (
                    tile.edge_vector(0),
                    tile.edge_vector(1),
                    tile.edge_vector(2),
                    tile.edge_vector(3),
                ),
            )
        })
        .collect::<HashMap<_, _>>()
}

#[allow(clippy::type_complexity)]
fn get_neighbors(
    edges: &HashMap<usize, (Vec<bool>, Vec<bool>, Vec<bool>, Vec<bool>)>,
) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut neighbors = HashMap::new();

    for (id1, e1) in edges {
        // match helper function
        let mtch = |v: &Vec<bool>, e: &(Vec<bool>, Vec<bool>, Vec<bool>, Vec<bool>)| {
            *v == e.0 || *v == e.1 || *v == e.2 || *v == e.3
        };

        // .0=left, .1=right, .2=top, .3=bottom));
        let mut vs = vec![];
        for (id2, e2) in edges {
            if e1 == e2 {
                continue;
            }

            if mtch(&e1.0, e2) || mtch(&e1.0.iter().rev().copied().collect(), e2) {
                vs.push((0, *id2));
            } else if mtch(&e1.1, e2) || mtch(&e1.1.iter().rev().copied().collect(), e2) {
                vs.push((1, *id2));
            } else if mtch(&e1.2, e2) || mtch(&e1.2.iter().rev().copied().collect(), e2) {
                vs.push((2, *id2));
            } else if mtch(&e1.3, e2) || mtch(&e1.3.iter().rev().copied().collect(), e2) {
                vs.push((3, *id2));
            }
        }
        neighbors.insert(*id1, vs);
    }
    neighbors
}

fn solution1(data: &[String]) -> usize {
    let tiles = get_tiles(data);
    let edges = get_edges(&tiles);
    // corners are unique with 2 neighbors
    get_neighbors(&edges)
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(id, _)| id)
        .product()
}

// given an ordered edge, lookup and print the matix of id's
#[allow(dead_code)]
fn debug_print_id(ordered_edge_ids: &[usize], neighbors: &HashMap<usize, Vec<(usize, usize)>>) -> Vec<usize> {
    let mut row = ordered_edge_ids.to_owned();
    let mut visited = ordered_edge_ids.to_owned();

    // neighbors is a pairwise lookup of id => List of adjacent matching details
    // the length of the list is the number of adjacents each described as a tuple
    //  (
    //      matched id,
    //      edge position of the current "data order - as read": .0=left, .1=right, .2=top, .3=bottom
    //  )

    // sqrt to get size
    let sz = (neighbors.len() as f64).sqrt() as usize;

    // collect the ids of the next row by walking the given row looking for the edge not yet visited
    for _ in 0..sz - 1 {
        let mut tmp = vec![];
        for id in &row {
            let adj = neighbors[id]
                .iter()
                .find(|(_, adj_id)| !visited.contains(adj_id))
                .unwrap();
            visited.push(adj.1);
            tmp.push(adj.1);
        }
        row = tmp;
    }

    // display an id grid
    for i in 0..sz {
        println!(
            "{}",
            visited
                .iter()
                .skip(i * sz)
                .take(sz)
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("\t")
        );
    }
    visited
}

#[allow(dead_code)]
fn debug_print_grid(grid: &Array2<bool>) {
    for r in 0..grid.nrows() {
        for c in 0..grid.ncols() {
            if grid[[r, c]] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
    let grid = flip_horizontal_axis(grid);
    for r in 0..grid.nrows() {
        for c in 0..grid.ncols() {
            if grid[[r, c]] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[rustfmt::skip]
fn solution2(data: &[String]) -> usize {
    let mut tiles = get_tiles(data);
    let edges = get_edges(&tiles);
    let neighbors = get_neighbors(&edges);
    let sz = (edges.len() as f64).sqrt() as usize;

    //                  #
    //#    ##    ##    ###
    // #  #  #  #  #  #
    let mut monster = Array::from_elem((3, 20), false);
    monster[[0, 18]] = true;
    monster[[1, 0]] = true;
    monster[[1, 5]] = true;
    monster[[1, 6]] = true;
    monster[[1, 11]] = true;
    monster[[1, 12]] = true;
    monster[[1, 17]] = true;
    monster[[1, 18]] = true;
    monster[[1, 19]] = true;
    monster[[2, 1]] = true;
    monster[[2, 4]] = true;
    monster[[2, 7]] = true;
    monster[[2, 10]] = true;
    monster[[2, 13]] = true;
    monster[[2, 16]] = true;
    let monster_true_count = monster.iter().filter(|b| **b).count();

    // there are 4 tiles with 2 adjacents (the corners)
    let corners = neighbors.iter().filter(|(_, v)| v.len() == 2).collect::<Vec<_>>();
    assert_eq!(corners.len(), 4);

    // select one of the corners to logically be top-left with adjacents "right" and "down"
    let random_corner = rand::thread_rng().gen_range(0..corners.len());
    let (id, adjacents) = corners[random_corner];

    let tile = &tiles[id].mat;
    match (adjacents[0].0, adjacents[1].0) {
        // the current "data read" order: .0=left, .1=right, .2=top, .3=bottom));
        (l, t) if (l, t) == (0, 2) => { tiles.insert(*id, Tile { mat: rotate_flip(0, true, true, tile) }); }
        (t, l) if (t, l) == (2, 0) => { tiles.insert(*id, Tile { mat: rotate_flip(90, true, false, tile) }); }

        (l, b) if (l, b) == (0, 3) => { tiles.insert(*id, Tile { mat: rotate_flip(0, false, true, tile) }); }
        (b, l) if (b, l) == (3, 0) => { tiles.insert(*id, Tile { mat: rotate_flip(-90, false, false, tile) }); }

        (r, t) if (r, t) == (1, 2) => { tiles.insert(*id, Tile { mat: rotate_flip(0, true, false, tile) }); }
        (t, r) if (t, r) == (2, 1) => { tiles.insert(*id, Tile { mat: rotate_flip(90, false, false, tile) }); }

        (r, b) if (r, b) == (1, 3) => { tiles.insert(*id, Tile { mat: rotate_flip(0, false, false, tile) }); }
        (b, r) if (b, r) == (3, 1) => { tiles.insert(*id, Tile { mat: rotate_flip(-90, true, false, tile) }); }
        _ => unreachable!(),
    }

    let mut id = *id;
    let mut first_row = vec![id];
    let mut right_edge = adjacents[0].0;

    for _ in 0..sz - 1 {
        let right_id = neighbors[&id]
            .iter()
            .filter(|(e, _)| *e == right_edge)
            .map(|(_, id)| *id)
            .next()
            .unwrap();
        let left_edge = neighbors[&right_id]
            .iter()
            .find(|(_, i)| *i == id)
            .map(|(e, _)| *e)
            .unwrap();

        // align tile
        let flip = tiles[&id].edge_vector(1) != tiles[&right_id].edge_vector(left_edge);
        let tile = &tiles[&right_id].mat;
        match left_edge {
            0 => { tiles.insert(right_id, Tile { mat: rotate_flip(0, flip, false, tile) }); }
            1 => { tiles.insert(right_id, Tile { mat: rotate_flip(0, flip, true, tile) }); }
            2 => { tiles.insert(right_id, Tile { mat: rotate_flip(-90, !flip, false, tile) }); }
            3 => { tiles.insert(right_id, Tile { mat: rotate_flip(90, flip, false, tile) }); }
            _ => unreachable!(),
        }

        right_edge = match left_edge {
            0 => 1, // left -> right
            1 => 0, // right -> left
            2 => 3, // top -> bottom
            3 => 2, // bottom -> top
            _ => unreachable!(),
        };

        id = right_id;
        first_row.push(id);
    }

    //debug_print_id(&first_row, &neighbors);

    let mut row = first_row.clone();
    let mut visited = first_row.clone();

    // collect the ids of the next row by walking the given row looking for the edge not yet visited
    for _ in 0..sz - 1 {
        let mut tmp = vec![];
        for tid in &row {
            let adj = neighbors[tid]
                .iter()
                .find(|(_, adj_id)| !visited.contains(adj_id))
                .unwrap();
            let down_id = adj.1;
            let top_edge = neighbors[&down_id]
                .iter()
                .find(|(_, i)| i == tid)
                .map(|(e, _)| *e)
                .unwrap();

            let flip = tiles[tid].edge_vector(3) != tiles[&down_id].edge_vector(top_edge);
            let tile = &tiles[&down_id].mat;
            match top_edge {
                0 => { tiles.insert( down_id, Tile { mat: rotate_flip(90, false, !flip, tile) }); }
                1 => { tiles.insert( down_id, Tile { mat: rotate_flip(-90, false, flip, tile) }); }
                2 => { tiles.insert( down_id, Tile { mat: rotate_flip(0, false, flip, tile) }); }
                3 => { tiles.insert( down_id, Tile { mat: rotate_flip(0, true, flip, tile) }); }
                _ => unreachable!(),
            }
            visited.push(down_id);
            tmp.push(down_id);
        }
        row = tmp;
    }

    // everything is rotated, grab the inner data (remove the border)
    let tile = tiles[&visited[0]].inner_data();
    let mut grid = Array::from_elem((sz * tile.nrows(), sz * tile.ncols()), false);
    let mut r = 0;
    let mut c = 0;
    for id in visited {
        let array = tiles[&id].inner_data();
        for i in 0..array.nrows() {
            for j in 0..array.ncols() {
                if array[[i, j]] {
                    grid[[r + i, c + j]] = true;
                }
            }
        }
        c += array.ncols();

        if c == grid.ncols() {
            r += array.nrows();
            c = 0;
        }
    }

    //debug_print_grid(&grid);

    // look for the sea monster
    let orig_grid = grid.clone();
    let true_count = grid.iter().filter(|b| **b).count();
    let mut max_monsters = 0;

    for rot in [0, 90, 180, 270] {
        for vflip in [false, true] {
            let mut mcount = 0;
            grid = rotate_flip(rot, false, vflip, &orig_grid.to_owned());
            for r in 0..grid.nrows() - monster.nrows() {
                for c in 0..grid.ncols() - monster.ncols() {
                    let mut count = 0;
                    for i in 0..monster.nrows() {
                        for j in 0..monster.ncols() {
                            if monster[[i, j]] && grid[[r + i, c + j]] {
                                count += 1;
                            }
                        }
                    }
                    if count == monster_true_count {
                        mcount += 1;
                    }
                }
            }
            max_monsters = max_monsters.max(mcount);
        }
    }
    true_count - max_monsters * monster_true_count
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

    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&puzzle_lines))?;

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
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(20899048083289, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(60145080587029, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(273, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(1901, solution2(&data));
    }
}
