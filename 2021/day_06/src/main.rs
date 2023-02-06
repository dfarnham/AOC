use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::error::Error;
use std::io::{self, Write};

fn cycle(data: &[u8], days: u32) -> usize {
    let counts = data.iter().collect::<Counter<_>>();
    let mut state = [
        counts[&0], counts[&1], counts[&2], counts[&3], counts[&4], counts[&5], counts[&6], counts[&7], counts[&8],
    ];
    for _ in 0..days {
        state = [
            state[1],
            state[2],
            state[3],
            state[4],
            state[5],
            state[6],
            state[7] + state[0],
            state[8],
            state[0],
        ];
    }
    state.iter().sum::<_>()
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

    let data = trim_split_on::<u8>(&puzzle_lines[0], ',')?;

    writeln!(stdout, "Answer Part 1 = {}", cycle(&data, 80))?;
    writeln!(stdout, "Answer Part 2 = {}", cycle(&data, 256))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_filedata(filename: &str) -> Vec<u8> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        trim_split_on::<u8>(&data[0], ',').unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_filedata("input-example");
        assert_eq!(cycle(&data, 18), 26);
        assert_eq!(cycle(&data, 80), 5934);
    }

    #[test]
    fn part1_actual() {
        let data = get_filedata("input-actual");
        assert_eq!(cycle(&data, 80), 358214);
    }

    #[test]
    fn part2_example() {
        let data = get_filedata("input-example");
        assert_eq!(cycle(&data, 256), 26984457539);
    }

    #[test]
    fn part2_actual() {
        let data = get_filedata("input-actual");
        assert_eq!(cycle(&data, 256), 1622533344325);
    }
}
