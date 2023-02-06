// This is just an implementation of: https://github.com/betaveros/advent-of-code-2022/blob/main/p19.noul
//
use general::{get_args, read_data_lines, reset_sigpipe};
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> Vec<Vec<usize>> {
    let mut values = vec![];
    let blueprint_re =
        Regex::new(r".*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+)").unwrap();
    for line in data {
        if blueprint_re.is_match(line) {
            let captures = blueprint_re.captures(line).unwrap();
            let nums = (1..=7)
                .map(|n| {
                    captures
                        .get(n)
                        .map(|s| s.as_str().parse::<usize>().unwrap())
                        .unwrap()
                })
                .collect::<Vec<_>>();
            values.push(nums);
        }
    }
    values
}

#[rustfmt::skip]
fn dfs(
    minute: usize,
    max_ore_cost: usize,
    costs: &[[usize;4]],
    resources: &[usize],
    bots: &[usize],
    ans: usize,
) -> usize {

    let ans_idle = resources[3] + bots[3] * minute;
    let mut ans = if ans_idle > ans { ans_idle } else { ans };

    if minute > 0 {
        let ans_opti = ans_idle + (minute * (minute - 1) / 2);
        if ans_opti <= ans {
            return ans;
        }
    }

    let turns_to_do = |cost: &[usize]| -> Option<usize> {
        // create tuple (resource, bot, cost)
        let ts = resources.iter()
            .zip(bots.iter())
            .zip(cost.iter())
            .map(|((r, b), c)| {
                if r >= c {
                    Some(0)
                } else if *b > 0 {
                    Some((c - r + b - 1) / b)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match ts.contains(&None) {
            true => None,
            false => *ts.iter().max().unwrap()
        }
    };

    for (i, c) in costs.iter().enumerate() {
        //                   >= max(ore_bot_cost, clay_bot_cost, obs_bot_ore_cost, geode_bot_ore_cost)
        if i == 0 && bots[i] >= max_ore_cost { continue; }

        //                   >= obs_bot_clay_cost
        if i == 1 && bots[i] >= costs[2][1] { continue; }

        //                   >= geode_bot_obs_cost
        if i == 2 && bots[i] >= costs[3][2] { continue; }

        if let Some(t) = turns_to_do(c) {
            if t < minute {
                let resources_adj = (0..bots.len())
                    .map(|j| resources[j] + (t + 1) * bots[j] - c[j])
                    .collect::<Vec<_>>();

                let mut bots_adj = bots.to_vec();
                bots_adj[i] += 1;

                ans = dfs(minute - t - 1, max_ore_cost, costs, &resources_adj, &bots_adj, ans);
            }
        }
    }
    ans
}

fn score(blueprint: Vec<usize>, cycles: usize) -> usize {
    let (
        _bp_id,
        ore_bot_cost,
        clay_bot_cost,
        obs_bot_ore_cost,
        obs_bot_clay_cost,
        geode_bot_ore_cost,
        geode_bot_obs_cost,
    ) = (
        blueprint[0],
        blueprint[1],
        blueprint[2],
        blueprint[3],
        blueprint[4],
        blueprint[5],
        blueprint[6],
    );

    let max_ore_cost = *[
        ore_bot_cost,
        clay_bot_cost,
        obs_bot_ore_cost,
        geode_bot_ore_cost,
    ]
    .iter()
    .max()
    .unwrap();

    // ordered by bot id
    let costs = [
        [ore_bot_cost, 0, 0, 0],
        [clay_bot_cost, 0, 0, 0],
        [obs_bot_ore_cost, obs_bot_clay_cost, 0, 0],
        [geode_bot_ore_cost, 0, geode_bot_obs_cost, 0],
    ];

    let resources = vec![0, 0, 0, 0];
    let bots = vec![1, 0, 0, 0];
    let ans = 0;

    dfs(cycles, max_ore_cost, &costs, &resources, &bots, ans)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let values = get_data(puzzle_lines);
    Ok(values.into_iter().map(|v| v[0] * score(v, 24)).sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let values = get_data(puzzle_lines);
    let values = match values.len() {
        // example (blueprint #2)
        2 => values[1..2].to_vec(),
        // actual (first 3 blueprints)
        _ => values[0..3].to_vec(),
    };
    Ok(values.into_iter().map(|v| score(v, 32)).product())
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
        assert_eq!(part1(&puzzle_lines)?, 33);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1616);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 62);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 8990);
        Ok(())
    }
}
