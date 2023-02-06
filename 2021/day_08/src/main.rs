use counter::Counter;
use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use itertools::Itertools;
use std::error::Error;
use std::io::{self, Write};

/*
      0:        1:        2:        3:        4:        5:        6:        7:        8:        9:
     aaaa      ....      aaaa      aaaa      ....      aaaa      aaaa      aaaa      aaaa      aaaa
    b    c    .    c    .    c    .    c    b    c    b    .    b    .    .    c    b    c    b    c
    b    c    .    c    .    c    .    c    b    c    b    .    b    .    .    c    b    c    b    c
     ....      ....      dddd      dddd      dddd      dddd      dddd      ....      dddd      dddd
    e    f    .    f    e    .    .    f    .    f    .    f    e    f    .    f    e    f    .    f
    e    f    .    f    e    .    .    f    .    f    .    f    e    f    .    f    e    f    .    f
     gggg      ....      gggg      gggg      ....      gggg      gggg      ....      gggg      gggg

Len   6         2         5         5         4         5         6         3         7         6
*/

fn get_solution(garbled: &[String]) -> Vec<u8> {
    let mut digits = vec![u8::MAX; 14];

    fn garbled_str(garbled: &[String], digits: &[u8], n: u8) -> String {
        garbled[digits.iter().position(|&num| num == n).unwrap()].to_string()
    }

    // find 1, 4, 7, 8
    for (i, garb) in garbled.iter().enumerate() {
        match garb.len() {
            2 => digits[i] = 1,
            3 => digits[i] = 7,
            4 => digits[i] = 4,
            7 => digits[i] = 8,
            _ => (),
        }
    }

    // known: 1, 4, 7, 8
    //
    // "3" is a digit.len() == 5 that contains the "1" chars
    let s = garbled_str(garbled, &digits, 1);
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 5 && s.chars().filter(|c| garb.contains(*c)).count() == s.len() {
            digits[i] = 3;
        }
    }

    // known: 1, 3, 4, 7, 8
    //
    // "9" is a digit.len() == 6 that contains the "3" chars
    let s = garbled_str(garbled, &digits, 3);
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 6 && s.chars().filter(|c| garb.contains(*c)).count() == s.len() {
            digits[i] = 9;
        }
    }

    // known: 1, 3, 4, 7, 8, 9
    //
    // "0" is a digit.len() == 6 that contains the "1" chars and is not the "9"
    let s = garbled_str(garbled, &digits, 1);
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 6 && s.chars().filter(|c| garb.contains(*c)).count() == s.len() && digits[i] != 9 {
            digits[i] = 0;
        }
    }

    // known: 0, 1, 3, 4, 7, 8, 9
    //
    // "6" is a digit.len() == 6 that is not 0, 9
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 6 && digits[i] != 0 && digits[i] != 9 {
            digits[i] = 6;
        }
    }

    // known: 0, 1, 3, 4, 6, 7, 8, 9
    //
    // "5" is a digit.len() == 5 that is contained within a "6"
    let s = garbled_str(garbled, &digits, 6);
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 5 && garb.chars().filter(|c| s.contains(*c)).count() == garb.len() {
            digits[i] = 5;
        }
    }

    // known: 0, 1, 3, 4, 5, 6, 7, 8, 9
    //
    // "2" is a digit.len() == 5 that is not 3, 5
    for (i, garb) in garbled.iter().enumerate() {
        if garb.len() == 5 && digits[i] != 3 && digits[i] != 5 {
            digits[i] = 2;
        }
    }

    digits[10..].to_vec()
}

fn get_segment_msg(data: &str) -> Vec<String> {
    data.split_whitespace()
        .filter(|s| *s != "|")
        .map(|s| s.trim().chars().sorted().collect::<_>())
        .collect::<Vec<_>>()
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

    let digits = puzzle_lines
        .iter()
        .map(|line| get_segment_msg(line))
        .collect::<Vec<Vec<_>>>();

    let mut digit_counts = Counter::<u8, usize>::new();
    let mut n = 0;
    for d in digits {
        let msg = get_solution(&d);
        // example: [3,4,5,6] into 3456
        //n += 1000 * msg[0] as u32 + 100 * msg[1] as u32 + 10 * msg[2] as u32 + msg[3] as u32;
        n += msg.iter().fold(0, |acc, x| 10 * acc + *x as u32);
        digit_counts += msg;
    }

    writeln!(
        stdout,
        "Answer Part 1 = {}",
        [1, 4, 7, 8].iter().map(|n| digit_counts[n]).sum::<usize>()
    )?;

    writeln!(stdout, "Answer Part 2 = {n}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datapoints(filename: &str) -> Vec<Vec<String>> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        data.iter().map(|line| get_segment_msg(line)).collect::<Vec<Vec<_>>>()
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        let mut digit_counts = Counter::<u8, usize>::new();
        for d in data {
            digit_counts += get_solution(&d);
        }
        assert_eq!([1, 4, 7, 8].iter().map(|n| digit_counts[n]).sum::<usize>(), 26);
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        let mut digit_counts = Counter::<u8, usize>::new();
        for d in data {
            digit_counts += get_solution(&d);
        }
        assert_eq!([1, 4, 7, 8].iter().map(|n| digit_counts[n]).sum::<usize>(), 445);
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        let mut n = 0;
        for d in data {
            let msg = get_solution(&d);
            n += msg.iter().fold(0, |acc, x| 10 * acc + *x as u32);
        }
        assert_eq!(n, 61229);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        let mut n = 0;
        for d in data {
            let msg = get_solution(&d);
            n += msg.iter().fold(0, |acc, x| 10 * acc + *x as u32);
        }
        assert_eq!(n, 1043101);
    }
}
