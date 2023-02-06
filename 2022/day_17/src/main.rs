use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Vec<char> {
    data[0].chars().collect::<Vec<char>>()
}

#[rustfmt::skip]
fn get_rocks(x_offset: usize) -> Vec<Vec<(usize, usize)>> {
    let rock1 = vec![ (0,0), (1,0), (2,0), (3,0) ];

    let rock2 = vec![        (1,2),
                      (0,1), (1,1), (2,1),
                             (1,0)         ];

    let rock3 = vec![               (2,2),
                                    (2,1),
                      (0,0), (1,0), (2,0)  ];

    let rock4 = vec![ (0,3),
                      (0,2),
                      (0,1),
                      (0,0)  ];

    let rock5 = vec![ (0,1), (1,1),
                      (0,0), (1,0) ];

    let rocks = vec![ rock1, rock2, rock3, rock4, rock5 ];
    rocks.iter()
        .map(|r| r.iter().map(|(x, y)| (*x + x_offset, *y)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn move_left(chamber: &HashSet<(usize, usize)>, rock: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if rock.iter().map(|(x, _)| *x).min() != Some(0) {
        let moved = rock
            .iter()
            .map(|(x, y)| (*x - 1, *y))
            .collect::<Vec<(_, _)>>();
        if legal_move(chamber, &moved) {
            return moved;
        }
    }
    rock.to_vec()
}

fn move_right(chamber: &HashSet<(usize, usize)>, rock: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if rock.iter().map(|(x, _)| *x).max() != Some(6) {
        let moved = rock
            .iter()
            .map(|(x, y)| (*x + 1, *y))
            .collect::<Vec<(_, _)>>();
        if legal_move(chamber, &moved) {
            return moved;
        }
    }
    rock.to_vec()
}

fn move_down(rock: &mut [(usize, usize)]) {
    for (_, y) in &mut rock.iter_mut() {
        *y -= 1;
    }
}

fn move_up(rock: &mut [(usize, usize)]) {
    for (_, y) in &mut rock.iter_mut() {
        *y += 1;
    }
}

fn adjust_height(rock: &[(usize, usize)], height: usize) -> Vec<(usize, usize)> {
    rock.iter()
        .map(|(x, y)| (*x, *y + height))
        .collect::<Vec<_>>()
}

fn legal_move(chamber: &HashSet<(usize, usize)>, rock: &[(usize, usize)]) -> bool {
    for (x, y) in rock {
        if chamber.contains(&(*x, *y)) {
            return false;
        }
    }
    true
}

fn solve(puzzle_lines: &[String], n: usize) -> Result<usize, Box<dyn Error>> {
    let jets = get_data(puzzle_lines);
    let rocks = get_rocks(2);
    let mut seen = HashMap::new();
    let mut chamber: HashSet<(usize, usize)> = (0..7).map(|x| (x, 0)).collect();

    let mut offset = 0;
    let mut highpoint = 0;
    let mut i = 0;
    let mut count = 0;
    while count < n {
        let rock_index = count % rocks.len();
        let mut rock = adjust_height(&rocks[rock_index], highpoint + 4);

        loop {
            rock = match jets[i] == '<' {
                true => move_left(&chamber, &rock),
                false => move_right(&chamber, &rock),
            };
            move_down(&mut rock);

            i = (i + 1) % jets.len();
            if !legal_move(&chamber, &rock) {
                move_up(&mut rock);
                for (x, y) in &rock {
                    chamber.insert((*x, *y));
                }

                highpoint = chamber.iter().map(|(_, y)| *y).max().unwrap();

                // build a creative hash key (unclear to me what a stable magic value should be)
                let magic = 0;
                let key = (
                    count % rocks.len(),
                    i,
                    chamber
                        .iter()
                        .filter(|(_, y)| highpoint - *y <= magic)
                        .map(|(x, y)| (*x, highpoint - *y))
                        .collect::<Vec<(_, _)>>(),
                );

                if seen.contains_key(&key) {
                    let (prev_count, prev_highpoint) = seen[&key];
                    let diff_count = count - prev_count;
                    let t = (n - count) / diff_count;
                    count += t * diff_count;
                    offset += t * (highpoint - prev_highpoint);
                    seen.clear();
                }

                seen.insert(key, (count, highpoint));
                break;
            }
        }
        count += 1;
    }
    Ok(highpoint + offset)
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

    writeln!(stdout, "Answer Part 1 = {}", solve(&puzzle_lines, 2022)?)?;
    writeln!(
        stdout,
        "Answer Part 2 = {}",
        solve(&puzzle_lines, 1000000000000)?
    )?;

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
        assert_eq!(solve(&puzzle_lines, 2022)?, 3068);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(solve(&puzzle_lines, 2022)?, 3111);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(solve(&puzzle_lines, 1000000000000)?, 1514285714288);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(solve(&puzzle_lines, 1000000000000)?, 1526744186042);
        Ok(())
    }
}
