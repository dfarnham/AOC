use general::{get_args, read_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Vec<i64> {
    let mut values = vec![];
    for line in data {
        values.push(line.parse::<i64>().unwrap());
    }
    values
}

fn solve(list: &[i64], offsets: &[usize], key: i64, rounds: usize) -> Result<i64, Box<dyn Error>> {
    // create a list of (prev, next) indicies and join the ends
    // ex. a list of len 4 => [(3, 1), (0, 2), (1, 3), (2, 0)]
    let mut clist = (0..list.len())
        .map(|i| match i {
            i if i == 0 => (list.len() - 1, i + 1),
            i if i == list.len() - 1 => (i - 1, 0),
            _ => (i - 1, i + 1),
        })
        .collect::<Vec<_>>();

    // will be set to the index where value 0 is encountered
    let mut zero_index = None;

    for _ in 0..rounds {
        for (i, v) in list.iter().enumerate() {
            if *v == 0 {
                zero_index = Some(i);
                continue;
            }

            let n = key * v;
            let mut new_index = i;
            for _ in 0..n.unsigned_abs() as usize % (list.len() - 1) {
                new_index = match n > 0 {
                    true => clist[new_index].1,  // right
                    false => clist[new_index].0, // left
                };
            }

            // move one more left so the updates below are the same
            if n < 0 {
                new_index = clist[new_index].0;
            }

            if new_index != i {
                // update neighbors of clist[i]
                let (l, r) = (clist[i].0, clist[i].1);
                (clist[l].1, clist[r].0) = (clist[i].1, clist[i].0);

                // update clist[i]
                (clist[i].0, clist[i].1) = (new_index, clist[new_index].1);

                // update the new neighbors
                let (l, r) = (clist[new_index].1, new_index);
                (clist[l].0, clist[r].1) = (i, i);
            }
        }
    }

    assert!(zero_index.is_some());
    let mut index = zero_index.unwrap();

    // a bit wasteful but helped in viewing the reordered list
    // build a new list starting at the zero index following to the right
    let mut olist = vec![];
    for _ in 0..clist.len() {
        olist.push(list[index]);
        index = clist[index].1; // right
    }

    Ok(key * offsets.iter().map(|n| olist[*n % olist.len()]).sum::<i64>())
}

fn part1(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let values = get_data(puzzle_lines);
    let (key, rounds) = (1, 1);
    solve(&values, &[1000, 2000, 3000], key, rounds)
}

fn part2(puzzle_lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let values = get_data(puzzle_lines);
    let (key, rounds) = (811589153, 10);
    solve(&values, &[1000, 2000, 3000], key, rounds)
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
        assert_eq!(part1(&puzzle_lines)?, 3);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 872);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 1623178306);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 5382459262696);
        Ok(())
    }
}
