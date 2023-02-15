use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn part1(puzzle_lines: &[String], rows: usize, columns: usize) -> Result<usize, Box<dyn Error>> {
    let chars = puzzle_lines[0].chars().collect::<Vec<_>>();

    // sanity check
    let sz = rows * columns;
    assert_eq!(chars.len() % sz, 0);

    // HashMap of HashMaps for each (row x column) layer
    let mut layers = HashMap::new();

    // compute char frequencies for each layer
    for (i, c) in chars.iter().enumerate() {
        // the layer associated with the data
        let key = i / rows / columns;

        // add a new layer map if it doesn't exist
        layers.entry(key).or_insert_with(HashMap::new);

        // update char frequency in each layer map
        let layer_map = layers.get_mut(&key).expect("invalid layer");
        *layer_map.entry(*c).or_insert(0) += 1;
    }

    // find the layer with smallest count of '0', then return
    // the count of '1' multiplied by the count of '2' in that layer
    Ok(match layers.iter().min_by(|a, b| a.1.get(&'0').cmp(&b.1.get(&'0'))) {
        Some(layer) => layer.1.get(&'1').unwrap_or(&0) * layer.1.get(&'2').unwrap_or(&0),
        None => panic!("no solution"),
    })
}

fn part2(puzzle_lines: &[String], rows: usize, columns: usize) -> Result<Vec<char>, Box<dyn Error>> {
    let chars = puzzle_lines[0].chars().collect::<Vec<_>>();

    // sanity check
    let sz = rows * columns;
    assert_eq!(chars.len() % sz, 0);

    // initialize image, filled with transparent '2'
    let mut image = vec!['2'; sz];
    for (i, c) in chars.iter().enumerate() {
        // overlay each layer, changing only transparents
        if image[i % sz] == '2' {
            image[i % sz] = *c;
        }
    }

    // output image ascii art
    for (i, c) in image.iter().enumerate() {
        if i > 0 && i % columns == 0 {
            println!();
        }
        print!("{}", if *c == '1' { '\u{25a0}' } else { ' ' });
    }
    println!();

    Ok(image)
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines, 6, 25)?)?;
    writeln!(stdout, "Answer Part 2 = {:?}", part2(&puzzle_lines, 6, 25)?)?;

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
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines, 6, 25)?, 1792);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        let message = [
            '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '1', '1', '1', '1', '0', '0', '1', '1', '0', '0', '1',
            '0', '0', '1', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '0', '0', '0', '0', '1', '0',
            '0', '1', '0', '1', '0', '0', '1', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1',
            '0', '0', '1', '0', '0', '0', '0', '1', '1', '1', '1', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1',
            '0', '1', '0', '0', '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1', '0', '1', '0', '0', '0', '0',
            '1', '0', '0', '1', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1', '0', '1', '0', '0', '1', '0', '1',
            '1', '1', '1', '0', '0', '1', '1', '0', '0', '1', '1', '1', '1', '0', '0', '1', '1', '0', '0', '1', '0',
            '0', '1', '0',
        ];
        assert_eq!(part2(&puzzle_lines, 6, 25)?, message);
        Ok(())
    }
}
