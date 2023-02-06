use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
enum Item {
    File(PathBuf, usize),
}
impl Item {
    fn size(&self) -> usize {
        match self {
            Self::File(_, size) => *size,
        }
    }
}

#[rustfmt::skip]
fn build_filesystem_view(commands: &[String]) -> Result<BTreeMap<String, Item>, Box<dyn Error>> {
    let mut fs = BTreeMap::new();
    let mut root = PathBuf::new();
    root.push("/");
    fs.insert(root.display().to_string(), Item::File(root.clone(), 0));

    for line in commands {
        if line.starts_with("$ ls") {
            continue;
        }

        let mut path = PathBuf::new();
        path.push(root.clone());

        // hash key: path as a string
        let hashkey = |path: PathBuf| path.display().to_string();

        if line.starts_with("$ cd") {
            // update path
            match &line[5..] {
                arg if arg == ".." => { path.pop(); }
                arg => { path.push(arg); }
            }

            // set the root pointer from the path
            let pathkey = hashkey(path.clone());
            root = match fs.get(&pathkey) {
                Some(Item::File(path, _)) => path.to_path_buf(),
                _ => return Err(Box::from(format!("Unknown directory: {pathkey}"))),
            };
        } else {
            let mut listing = line.split_whitespace();
            if let (Some(attr), Some(name)) = (listing.next(), listing.next()) {
                // update path
                path.push(name);

                // insert the full path of the item with a size: dir(0) or filesize
                let pathkey = hashkey(path.clone());
                let size = match attr == "dir" {
                    true => 0,
                    false => attr.parse::<usize>()?,
                };
                fs.insert(pathkey, Item::File(path, size));
            }
        }
    }

    Ok(fs)
}

fn get_dir_size(dir: &str, fs: &BTreeMap<String, Item>) -> usize {
    fs.iter()
        .filter(|(path, _)| path.starts_with(dir))
        .map(|(_, item)| item.size())
        .sum::<usize>()
}

fn get_dir_sizes(fs: &BTreeMap<String, Item>) -> Vec<usize> {
    fs.iter()
        .map(|(path, _)| get_dir_size(&(path.to_owned() + "/"), fs))
        .collect::<Vec<_>>()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let fs = build_filesystem_view(puzzle_lines)?;
    Ok(get_dir_sizes(&fs)
        .iter()
        .filter(|s| *s <= &100000)
        .sum::<usize>())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let fs = build_filesystem_view(puzzle_lines)?;
    let free = 70000000 - get_dir_size("/", &fs);
    Ok(get_dir_sizes(&fs)
        .iter()
        .filter(|s| free + *s >= 30000000)
        .copied()
        .min()
        .expect("no solution"))
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
        assert_eq!(part1(&puzzle_lines)?, 95437);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1778099);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 24933642);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1623571);
        Ok(())
    }
}
