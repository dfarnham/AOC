use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::HashMap;

//                            q r s
//
//                       / \         / \
//                     /     \     /     \
//                   /  0      \ /  +1     \
//                   |      -1 | |      -1 |
//                   |         | |         |
//                   \  +1     / \  0      /
//                 / \ \     / / \ \     / / \
//               /     \ \ / /     \ \ / /     \
//             /  -1     \ /  q      \ / +1      \
//             |       0 | |       r | |       0 |
//             |         | |         | |         |
//             \  +1     / \  s      / \  -1     /
//               \     / / \ \     / / \ \     /
//                 \ / /     \ \ / /     \ \ /
//                   /  -1     \ /  0      \
//                   |      +1 | |      +1 |
//                   |         | |         |
//                   \  0      / \  -1     /
//                     \     /     \     /
//                       \ /         \ /
//

const W: (i8, i8, i8) = (-1, 0, 1);
const E: (i8, i8, i8) = (1, 0, -1);

const NW: (i8, i8, i8) = (0, -1, 1);
const SE: (i8, i8, i8) = (0, 1, -1);

const NE: (i8, i8, i8) = (1, -1, 0);
const SW: (i8, i8, i8) = (-1, 1, 0);

fn get_data(data: &[String]) -> Vec<Vec<(i8, i8, i8)>> {
    let mut all_lines = vec![];
    for line in data {
        let mut i = 0;
        let mut moves = vec![];
        while i < line.len() {
            if line[i..].starts_with("ne") {
                moves.push(NE);
                i += 2;
            } else if line[i..].starts_with("nw") {
                moves.push(NW);
                i += 2;
            } else if line[i..].starts_with("se") {
                moves.push(SE);
                i += 2;
            } else if line[i..].starts_with("sw") {
                moves.push(SW);
                i += 2;
            } else if line[i..].starts_with('e') {
                moves.push(E);
                i += 1;
            } else if line[i..].starts_with('w') {
                moves.push(W);
                i += 1;
            } else {
                unreachable!();
            }
        }
        all_lines.push(moves);
    }
    all_lines
}

fn flipped_tiles(data: &[String]) -> HashMap<(i8, i8, i8), bool> {
    let mut tiles = HashMap::new();
    let origin = (0, 0, 0);

    tiles.insert(origin, false);

    for line in get_data(data) {
        let mut tile = origin;
        for (q, r, s) in line {
            tile.0 += q;
            tile.1 += r;
            tile.2 += s;
        }
        if let Some(state) = tiles.get(&tile) {
            tiles.insert(tile, !state);
        } else {
            tiles.insert(tile, true);
        }
    }

    tiles
}

fn solution1(data: &[String]) -> usize {
    let tiles = flipped_tiles(data);

    // count black
    tiles.into_iter().filter(|(_, state)| *state).count()
}

fn solution2(data: &[String]) -> usize {
    let mut tiles = flipped_tiles(data);

    for _ in 0..100 {
        let mut tiles_updated = tiles.clone();

        // fill in missing adjacents as white
        for tile in tiles.keys() {
            for direction in [W, E, NW, SE, NE, SW] {
                let (q, r, s) = direction;
                let adjacent = (tile.0 + q, tile.1 + r, tile.2 + s);
                tiles_updated.entry(adjacent).or_insert(false);
            }
        }
        tiles = tiles_updated.clone();

        for (tile, state) in &tiles {
            // count black adjacents
            let mut black = 0;
            for direction in [W, E, NW, SE, NE, SW] {
                let (q, r, s) = direction;
                let adjacent = (tile.0 + q, tile.1 + r, tile.2 + s);
                if tiles.get(&adjacent) == Some(&true) {
                    black += 1;
                }
            }

            // color flipping rules:
            // 1. Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white
            // 2. Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black
            if *state {
                if black == 0 || black > 2 {
                    tiles_updated.insert(*tile, false);
                }
            } else if black == 2 {
                tiles_updated.insert(*tile, true);
            } else if black == 0 {
                // trim some of adjacents to keep the hash smaller
                tiles_updated.remove(tile);
            }
        }
        tiles = tiles_updated;
    }

    // count black
    tiles.into_iter().filter(|(_, state)| *state).count()
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
        assert_eq!(10, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(307, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(2208, solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(3787, solution2(&data));
    }
}
