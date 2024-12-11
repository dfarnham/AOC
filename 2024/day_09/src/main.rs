use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    const RADIX: u32 = 10;
    let mut disk_map: Vec<_> = puzzle_lines[0]
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap() as usize)
        .collect();

    // create a compact disk representation to compute the checksum
    let mut compact_disk = vec![];
    let mut i = 0;
    let mut j = disk_map.len() - 1;

    while i <= j {
        let even = i % 2 == 0;
        for _ in 0..disk_map[i] {
            if even {
                compact_disk.push(i / 2);
            } else {
                if disk_map[j] == 0 {
                    j -= 2;
                }
                disk_map[j] -= 1;
                compact_disk.push(j / 2);
            }
        }
        i += 1;
    }

    // checksum
    Ok(compact_disk.iter().enumerate().map(|(i, n)| i * n).sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    const RADIX: u32 = 10;
    // tuple (id, len, free)
    let mut files: Vec<_> = puzzle_lines[0]
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .map(|(i, digits)| (i, digits[0], if digits.len() == 2 { digits[1] } else { 0 }))
        .collect();

    // process in decreasing file id order
    for id in (1..files.len()).rev() {
        // get the index of the candidate file to move
        let i = files.iter().rposition(|&tup| tup.0 == id).expect("fixme");

        // search for a previous file with enough free space
        if let Some(j) = files.iter().take(i).position(|&file| file.2 >= files[i].1) {
            // ------------------------------------
            // Move file[i] to the right of file[j]
            // ------------------------------------

            // remove the file[i] from its current location and add its length + free space
            // to the previous file[i-1], preserving the gap created.
            let mut file = files.remove(i);
            files[i - 1].2 += file.1 + file.2;

            // in its new location the free space will be the space remaining from file[j]
            file.2 = files[j].2 - file.1;
            files.insert(j + 1, file);

            // files[j]'s free space is now zero, any remaining was transferred to file[i] above
            files[j].2 = 0;
        }
    }

    // avoid padding the last file with zeros
    let last = files.len() - 1;
    files[last].2 = 0;

    // create a compact disk representation to compute the checksum
    let mut compact_disk = vec![];
    for f in files {
        compact_disk.resize(compact_disk.len() + f.1, f.0);
        compact_disk.resize(compact_disk.len() + f.2, 0);
    }

    // checksum
    Ok(compact_disk.iter().enumerate().map(|(i, n)| i * n).sum())
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
        assert_eq!(part1(&puzzle_lines)?, 1928);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 6241633730082);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 2858);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 6265268809555);
        Ok(())
    }
}
