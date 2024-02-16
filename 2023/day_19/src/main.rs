use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_on};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn evaluate(expr: &str, vars: &HashMap<String, usize>, logic_stmts: &HashMap<String, String>) -> bool {
    if expr == "A" {
        true
    } else if expr == "R" {
        false
    } else if let Some(e) = logic_stmts.get(expr) {
        evaluate(e, vars, logic_stmts)
    } else {
        let cmp = expr.split_once(':').unwrap();
        let branch = cmp.1.split_once(',').unwrap();
        let lt = cmp.0.contains('<');
        let (k, v) = match lt {
            true => cmp.0.split_once('<').unwrap(),
            false => cmp.0.split_once('>').unwrap(),
        };
        let n = v.parse::<usize>().unwrap();
        match lt && vars[k] < n || !lt && vars[k] > n {
            true => evaluate(branch.0, vars, logic_stmts),
            false => evaluate(branch.1, vars, logic_stmts),
        }
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut logic_stmts = HashMap::new();
    let mut first_half = true;
    let mut ans = 0;
    for line in puzzle_lines {
        if line.is_empty() {
            first_half = false;
            continue;
        }
        if first_half {
            let names = line.split_once('{').unwrap();
            let expr = names.1.replace('}', "");
            // ex. px{a<2006:qkq,m>2090:A,rfg} => (px, a<2006:qkq,m>2090:A,rfg)
            logic_stmts.insert(names.0.to_string(), expr.to_string());
        } else {
            let line = line.replace(['{', '}'], "");
            // ex. {x=787,m=2655,a=1222,s=2876} => (x, 787), (m, 2655), ..
            let mut vars = HashMap::new();
            for def in trim_split_on::<String>(&line, ',')? {
                let parts = def.split_once('=').unwrap();
                vars.insert(parts.0.to_string(), parts.1.parse::<usize>().unwrap());
            }
            if evaluate("in", &vars, &logic_stmts) {
                ans += vars.values().sum::<usize>();
            }
        }
    }
    Ok(ans)
}

fn new_range(op: &str, n: usize, lo: usize, hi: usize) -> (usize, usize) {
    let mut lo = lo;
    let mut hi = hi;
    match op {
        ">" => lo = lo.max(n + 1),
        "<" => hi = hi.min(n - 1),
        ">=" => lo = lo.max(n),
        "<=" => hi = hi.min(n),
        _ => unreachable!(),
    }
    (lo, hi)
}

#[allow(clippy::too_many_arguments)]
fn new_ranges(
    var: &str,
    op: &str,
    n: usize,
    xl: usize,
    xh: usize,
    ml: usize,
    mh: usize,
    al: usize,
    ah: usize,
    sl: usize,
    sh: usize,
) -> (usize, usize, usize, usize, usize, usize, usize, usize) {
    let mut xl = xl;
    let mut xh = xh;
    let mut ml = ml;
    let mut mh = mh;
    let mut al = al;
    let mut ah = ah;
    let mut sl = sl;
    let mut sh = sh;
    match var {
        "x" => (xl, xh) = new_range(op, n, xl, xh),
        "m" => (ml, mh) = new_range(op, n, ml, mh),
        "a" => (al, ah) = new_range(op, n, al, ah),
        "s" => (sl, sh) = new_range(op, n, sl, sh),
        _ => unreachable!(),
    }
    (xl, xh, ml, mh, al, ah, sl, sh)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut logic_stmts: HashMap<String, String> = HashMap::new();
    let mut ans = 0;
    for line in puzzle_lines {
        if line.is_empty() {
            break;
        }
        let names = line.split_once('{').unwrap();
        let expr = names.1.replace('}', "");
        logic_stmts.insert(names.0.to_string(), expr.to_string());
    }

    let mut workq = VecDeque::new();
    workq.push_back(("in", 1, 4000, 1, 4000, 1, 4000, 1, 4000));
    while let Some((expr, mut xl, mut xh, mut ml, mut mh, mut al, mut ah, mut sl, mut sh)) = workq.pop_front() {
        if expr == "R" || xl > xh || ml > mh || al > ah || sl > sh {
            continue;
        }
        if expr == "A" {
            ans += (xh - xl + 1) * (mh - ml + 1) * (ah - al + 1) * (sh - sl + 1);
            continue;
        } else if let Some(e) = logic_stmts.get(expr) {
            for cmd in e.split(',') {
                if cmd.contains(':') {
                    let t = cmd.split_once(':').unwrap();
                    let (cond, res) = (t.0, t.1);
                    let var = &cond[0..1];
                    let op = &cond[1..2];
                    let n = &cond[2..].parse::<usize>().unwrap();
                    let (xl2, xh2, ml2, mh2, al2, ah2, sl2, sh2) =
                        new_ranges(var, op, *n, xl, xh, ml, mh, al, ah, sl, sh);
                    workq.push_back((res, xl2, xh2, ml2, mh2, al2, ah2, sl2, sh2));
                    if op == "<" {
                        (xl, xh, ml, mh, al, ah, sl, sh) = new_ranges(var, ">=", *n, xl, xh, ml, mh, al, ah, sl, sh);
                    } else {
                        (xl, xh, ml, mh, al, ah, sl, sh) = new_ranges(var, "<=", *n, xl, xh, ml, mh, al, ah, sl, sh);
                    }
                } else {
                    workq.push_back((cmd, xl, xh, ml, mh, al, ah, sl, sh));
                }
            }
        } else {
            unreachable!()
        }
    }
    Ok(ans)
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
        assert_eq!(part1(&puzzle_lines)?, 19114);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 353553);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part2(&puzzle_lines)?, 167409079868000);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 124615747767410);
        Ok(())
    }
}
