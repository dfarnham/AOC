// using Floyd-Marshall as seen here:
// https://github.com/betaveros/advent-of-code-2022/blob/main/p16.noul
//
use general::{get_args, read_data_lines, reset_sigpipe};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> BTreeMap<String, (usize, Vec<String>)> {
    // ex.  Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    // ex.  Valve HH has flow rate=22; tunnel leads to valve GG
    let valve_re =
        Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)")
            .unwrap();
    let mut valves = BTreeMap::new();
    for line in data {
        if valve_re.is_match(line) {
            let captures = valve_re.captures(line).unwrap();
            let valve = captures.get(1).map(|s| s.as_str().to_string()).unwrap();
            let rate = captures
                .get(2)
                .map(|s| s.as_str().parse::<usize>().unwrap())
                .unwrap();
            let leads_to = captures
                .get(3)
                .map(|s| s.as_str().to_string())
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            valves.insert(valve, (rate, leads_to));
        }
    }
    valves
}

fn solve(puzzle_lines: &[String], part: usize) -> Result<usize, Box<dyn Error>> {
    let mut valves = vec![];
    let mut flows = vec![];
    let mut tunnels = HashMap::new();

    // get_data() returns this BTreeMap() on the example input
    // {"AA": (0, ["DD", "II", "BB"]),
    //  "BB": (13, ["CC", "AA"]),
    //  "CC": (2, ["DD", "BB"]),
    //  "DD": (20, ["CC", "AA", "EE"]),
    //  "EE": (3, ["FF", "DD"]),
    //  "FF": (0, ["EE", "GG"]),
    //  "GG": (0, ["FF", "HH"]),
    //  "HH": (22, ["GG"]),
    //  "II": (0, ["AA", "JJ"]),
    //  "JJ": (21, ["II"])}
    for (valve, (flow, neighbors)) in &get_data(puzzle_lines) {
        valves.push(valve.clone());
        flows.push(*flow);
        tunnels.insert(valve.clone(), neighbors.clone());
    }

    let mut dist = valves
        .iter()
        .map(|_| vec![valves.len(); valves.len()])
        .collect::<Vec<_>>();

    for (i, valve) in valves.iter().enumerate() {
        for neighbor in &tunnels[valve] {
            let list = &mut dist[i];
            list[valves
                .iter()
                .position(|s| s == neighbor)
                .expect("index bug")] = 1;
        }
    }

    // Floyd-Marshall https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    // non-zero flows
    let indices = flows
        .iter()
        .enumerate()
        .filter(|(_, &f)| f > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let cache1 = &mut HashMap::<(usize, Vec<usize>, usize), usize>::new();
    let cache2 = &mut HashMap::<(usize, Vec<usize>, usize), usize>::new();
    Ok(match part {
        1 => dfs1(&dist, &flows, 0, &indices, 30, cache1),
        2 => dfs2(&dist, &flows, 0, &indices, 26, cache1, cache2),
        _ => unreachable!(),
    })
}

// example: input &[1, 2, 3, 4] returns an iterator over:
//   (1, [2, 3, 4])
//   (2, [1, 3, 4])
//   (3, [1, 2, 4])
//   (4, [1, 2, 3])
fn choose_one(v: &[usize]) -> impl Iterator<Item = (&usize, Vec<usize>)> {
    v.iter().zip(
        (0..v.len())
            .map(|i| {
                [&v[..i], &v[i + 1..]]
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect()
            })
    )
}

fn dfs1(
    dist: &[Vec<usize>],
    flows: &[usize],
    cur: usize,
    indices: &[usize],
    time: usize,
    cache: &mut HashMap<(usize, Vec<usize>, usize), usize>,
) -> usize {
    if let Some(val) = cache.get(&(cur, indices.into(), time)) {
        return *val;
    }

    let mut maxval = 0;
    for (i, list) in choose_one(indices).filter(|(&i, _)| dist[cur][i] < time) {
        let dt = time - dist[cur][*i] - 1;
        maxval = maxval.max(flows[*i] * dt + dfs1(dist, flows, *i, &list, dt, cache));
    }

    cache.insert((cur, indices.into(), time), maxval);
    maxval
}

fn dfs2(
    dist: &[Vec<usize>],
    flows: &[usize],
    cur: usize,
    indices: &[usize],
    time: usize,
    dfs1_cache: &mut HashMap<(usize, Vec<usize>, usize), usize>,
    dfs2_cache: &mut HashMap<(usize, Vec<usize>, usize), usize>,
) -> usize {
    if let Some(val) = dfs2_cache.get(&(cur, indices.into(), time)) {
        return *val;
    }

    let mut maxval = 0;
    for (i, list) in choose_one(indices).filter(|(&i, _)| dist[cur][i] < time) {
        let dt = time - dist[cur][*i] - 1;
        maxval =
            maxval.max(flows[*i] * dt + dfs2(dist, flows, *i, &list, dt, dfs1_cache, dfs2_cache));
    }

    maxval = match dfs1_cache.get(&(0, indices.to_vec(), 26)) {
        Some(n) => maxval.max(*n),
        _ => maxval.max(dfs1(dist, flows, 0, indices, 26, dfs1_cache)),
    };

    dfs2_cache.insert((cur, indices.into(), time), maxval);
    maxval
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 2)
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
        assert_eq!(part1(&puzzle_lines)?, 1651);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1376);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 1707);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1933);
        Ok(())
    }
}
