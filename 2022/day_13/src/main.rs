use general::{get_args, read_data_lines, reset_sigpipe};
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

#[macro_use]
extern crate json;

type List = json::JsonValue;
// using json::JsonValue as a List abstraction
//
// the subset of JsonValue methods/macros used:
//    JsonValue.is_number()
//    JsonValue.is_array()
//    JsonValue.is_null()
//    JsonValue.as_u64()
//    macro array![] to create a new list

// consume the input data, returning a Vec of List pairs
fn get_data(data: &[String]) -> Vec<(List, List)> {
    let lists = data
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| json::parse(line).expect("unparsable List"))
        .collect::<Vec<_>>();

    assert!(lists.len() % 2 == 0, "expecting pairs");

    // create pairs
    lists
        .iter()
        .step_by(2)
        .zip(lists.iter().skip(1).step_by(2))
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect()
}

fn compare(left: &List, right: &List) -> Ordering {
    match (left, right) {
        (l, r) if l.is_number() && r.is_number() => l.as_u64().cmp(&r.as_u64()),
        (l, r) if l.is_array() && r.is_array() => {
            for i in 0..l.len().max(r.len()) {
                if l[i].is_null() && r[i].is_null() {
                    return Ordering::Equal;
                } else if l[i].is_null() {
                    return Ordering::Less;
                } else if r[i].is_null() {
                    return Ordering::Greater;
                }

                let ordering = compare(&l[i], &r[i]);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            Ordering::Equal
        }
        (l, r) if l.is_number() => compare(&array![l.as_u64()], r),
        (l, r) => {
            assert!(r.is_number());
            compare(l, &array![r.as_u64()])
        }
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(get_data(puzzle_lines)
        .iter()
        .enumerate()
        .map(|(i, p)| (i, compare(&p.0, &p.1)))
        .filter(|(_, c)| *c == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let markers = [array!([[2]]), array!([[6]])];

    let mut packets = get_data(puzzle_lines)
        .iter()
        .flat_map(|p| [p.0.clone(), p.1.clone()])
        .chain(markers.clone())
        .collect::<Vec<_>>();

    packets.sort_by(compare);

    Ok(packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| markers.contains(p))
        .map(|(i, _)| i + 1)
        .product())
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
        assert_eq!(part1(&puzzle_lines)?, 13);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 4734);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 140);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 21836);
        Ok(())
    }
}
