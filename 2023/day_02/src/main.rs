use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

// input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// output: (id, max_red, max_green, max_blue)
//         (1 /* game id */, max(4, 1, 0), max(0, 2, 2), max(3, 0, 6)
//         (1, 4, 2, 6)
fn id_max_vals(line: &str) -> (usize, usize, usize, usize) {
    let parts: Vec<_> = line.split(':').take(2).collect();
    let id = parts[0].split_whitespace().last().unwrap().parse::<usize>().unwrap();

    // we can remove all semi-colons and commas then split on whitespace and walk by pairs
    let grabs = parts[1].replace([';', ','], "");
    grabs
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|w| (w[0].parse::<usize>().unwrap(), w[1]))
        // [ (3, "blue"), (4, "red"), (1, "red"), (2, "green"), (6, "blue"), (2, "green") ]
        .fold((id, 0, 0, 0), |(id, red, green, blue), (n, color)| match color {
            "red" => (id, n.max(red), green, blue),
            "green" => (id, red, n.max(green), blue),
            "blue" => (id, red, green, n.max(blue)),
            _ => unreachable!(),
        })
}

// input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// output: (id, max_red, max_green, max_blue)
//         (1 /* game id */, max(4, 1, 0), max(0, 2, 2), max(3, 0, 6)
//         (1, 4, 2, 6)
#[allow(dead_code)]
fn id_max_vals_orig(line: &str) -> (usize, usize, usize, usize) {
    let parts: Vec<_> = line.split(':').take(2).collect();
    // [
    //    Game 1",
    //    "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // ]

    let id: String = parts[0].split_whitespace().skip(1).take(1).collect();
    parts[1]
        .split(';')
        // [
        //     "3 blue, 4 red",
        //     "1 red, 2 green, 6 blue",
        //     "2 green"
        // ]
        .flat_map(|grab| {
            grab.split(',')
                // [
                //     [ "3 blue", "4 red" ],
                //     [ "1 red", "2 green", "6 blue" ]
                //     [ "2 green" ]
                // ]
                .map(|s| s.split_whitespace().take(2).collect::<Vec<_>>())
                .map(|g| (g[0].parse::<usize>().unwrap(), g[1]))
        })
        // [ (3, "blue"), (4, "red"), (1, "red"), (2, "green"), (6, "blue"), (2, "green") ]
        .fold(
            (id.parse::<usize>().unwrap(), 0, 0, 0),
            |(id, red, green, blue), (n, color)| match color {
                "red" => (id, n.max(red), green, blue),
                "green" => (id, red, n.max(green), blue),
                "blue" => (id, red, green, n.max(blue)),
                _ => unreachable!(),
            },
        )
}
fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(puzzle_lines
        .iter()
        .map(|line| id_max_vals(line))
        .filter(|(_, red, green, blue)| red <= &12 && green <= &13 && blue <= &14)
        .map(|tup| tup.0)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(puzzle_lines
        .iter()
        .map(|line| id_max_vals(line))
        .map(|(_, red, green, blue)| red * green * blue)
        .sum())
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

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 8);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 2176);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 2286);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 63700);
        Ok(())
    }
}
