use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

const ROTATIONS: usize = 24;
// http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm

#[derive(Debug, Clone, PartialEq)]
struct Scanner {
    id: u8,
    beacons: Vec<(i64, i64, i64)>,
    distances: HashMap<(i64, i64, i64), usize>,
}
impl Scanner {
    fn new(id: u8, beacons: Vec<(i64, i64, i64)>) -> Self {
        let mut distances = HashMap::new();
        for i in 0..beacons.len() {
            for j in (i + 1)..beacons.len() {
                let tup = (
                    (beacons[i].0 - beacons[j].0),
                    (beacons[i].1 - beacons[j].1),
                    (beacons[i].2 - beacons[j].2),
                );
                assert!(!distances.contains_key(&tup));
                distances.insert(tup, i);
            }
        }

        Self { id, beacons, distances }
    }
}
impl Scanner {
    fn rotate(&self, rotation_index: usize) -> Self {
        let mut beacons = vec![];
        for (x, y, z) in &self.beacons {
            let (x, y, z) = (*x, *y, *z);
            let rotated_beacon = match rotation_index {
                0 => (x, y, z),
                1 => (x, -z, y),
                2 => (x, -y, -z),
                3 => (x, z, -y),
                4 => (-x, -y, z),
                5 => (-x, -z, -y),
                6 => (-x, y, -z),
                7 => (-x, z, y),
                8 => (y, x, -z),
                9 => (y, -x, z),
                10 => (y, z, x),
                11 => (y, -z, -x),
                12 => (-y, x, z),
                13 => (-y, -x, -z),
                14 => (-y, -z, x),
                15 => (-y, z, -x),
                16 => (z, x, y),
                17 => (z, -x, -y),
                18 => (z, -y, x),
                19 => (z, y, -x),
                20 => (-z, x, -y),
                21 => (-z, -x, y),
                22 => (-z, y, x),
                23 => (-z, -y, -x),
                _ => panic!("invalid index {rotation_index}"),
            };
            beacons.push(rotated_beacon);
        }
        Self::new(self.id, beacons)
    }
}
impl Scanner {
    fn relpos(&self, other: &Scanner) -> Option<(i64, i64, i64)> {
        let k1 = self.distances.keys().copied().collect::<HashSet<_>>();
        let k2 = other.distances.keys().copied().collect::<HashSet<_>>();
        let dkeys = k1.intersection(&k2).copied().collect::<HashSet<_>>();

        match dkeys.len() > 11 {
            true => {
                let tup = dkeys.iter().next().unwrap();
                let a = *self.distances.get(tup).unwrap();
                let b = *other.distances.get(tup).unwrap();
                Some((
                    self.beacons[a].0 - other.beacons[b].0,
                    self.beacons[a].1 - other.beacons[b].1,
                    self.beacons[a].2 - other.beacons[b].2,
                ))
            }
            false => None,
        }
    }
}

fn get_data(data: &[String]) -> Vec<Scanner> {
    let re = Regex::new(r"--\s+scanner\s+(-?\d+)").unwrap();
    let mut scanners = vec![];
    let mut beacons = vec![];
    let mut id = 0;
    for line in data.iter().filter(|s| !s.trim().is_empty()) {
        if line.contains("scanner") {
            if !beacons.is_empty() {
                scanners.push(Scanner::new(id, beacons));
            }
            let captures = re.captures(line).unwrap();
            id = captures.get(1).map(|s| s.as_str().parse::<u8>().unwrap()).unwrap();
            beacons = vec![];
            continue;
        }
        let coords = line
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(coords.len(), 3);
        beacons.push((coords[0], coords[1], coords[2]));
    }
    scanners.push(Scanner::new(id, beacons));
    scanners
}

fn solutions(scanners: &[Scanner]) -> (usize, u64) {
    let mut positions = vec![];
    let mut rotations = HashMap::new();

    for i in 0..scanners.len() {
        positions.push((0, 0, 0));
        rotations.insert(i, 0);
    }
    let mut solved = HashSet::new();
    let mut tested = HashSet::new();
    solved.insert(0);

    while solved.len() != scanners.len() {
        for i in solved.clone() {
            for j in 0..scanners.len() {
                if i == j || solved.contains(&j) || tested.contains(&(i, j)) || tested.contains(&(j, i)) {
                    continue;
                }
                tested.insert((i, j));
                let rscan = scanners[i].rotate(*rotations.get(&i).expect("no rotation"));
                for k in 0..ROTATIONS {
                    if let Some(pos) = rscan.relpos(&scanners[j].rotate(k)) {
                        rotations.insert(j, k);
                        solved.insert(j);
                        positions[j] = (pos.0 + positions[i].0, pos.1 + positions[i].1, pos.2 + positions[i].2);
                        break;
                    }
                }
            }
        }
    }

    let mut beacons = HashSet::<(i64, i64, i64)>::new();
    for i in 0..scanners.len() {
        for b in &scanners[i].rotate(*rotations.get(&i).expect("no rotation")).beacons {
            beacons.insert((b.0 + positions[i].0, b.1 + positions[i].1, b.2 + positions[i].2));
        }
    }

    let mut best = 0;
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            best = best.max(
                (positions[i].0 - positions[j].0).abs()
                    + (positions[i].1 - positions[j].1).abs()
                    + (positions[i].2 - positions[j].2).abs(),
            );
        }
    }
    (beacons.len(), best as u64)
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

    let scanners = get_data(&puzzle_lines);
    let (s1, s2) = solutions(&scanners);
    writeln!(stdout, "Answer Part 1 = {s1}")?;
    writeln!(stdout, "Answer Part 2 = {s2}")?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Vec<Scanner> {
        let file = std::path::PathBuf::from(filename);
        let data = read_trimmed_data_lines::<String>(Some(&file)).unwrap();
        get_data(&data)
    }

    #[test]
    fn part1_example() {
        let scanners = get_test_data("input-example");
        assert_eq!(solutions(&scanners).0, 79);
    }

    #[test]
    fn part1_actual() {
        let scanners = get_test_data("input-actual");
        assert_eq!(solutions(&scanners).0, 457);
    }

    #[test]
    fn part2_example() {
        let scanners = get_test_data("input-example");
        assert_eq!(solutions(&scanners).1, 3621);
    }

    #[test]
    fn part2_actual() {
        let scanners = get_test_data("input-actual");
        assert_eq!(solutions(&scanners).1, 13243);
    }
}
