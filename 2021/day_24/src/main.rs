use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
struct ALU {
    input: Vec<i64>,
    index: usize,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

fn expr(alu: &ALU, e: &str) -> i64 {
    match e {
        "w" => alu.w,
        "x" => alu.x,
        "y" => alu.y,
        "z" => alu.z,
        _ => e.parse::<i64>().unwrap(),
    }
}

fn compute(alu: &mut ALU, instructions: &[String]) {
    for line in instructions {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let expr_value = match fields[0] {
            "inp" => {
                alu.index += 1;
                alu.input[alu.index - 1]
            }
            "add" => expr(alu, fields[1]) + expr(alu, fields[2]),
            "mul" => expr(alu, fields[1]) * expr(alu, fields[2]),
            "div" => expr(alu, fields[1]) / expr(alu, fields[2]),
            "mod" => expr(alu, fields[1]) % expr(alu, fields[2]),
            "eql" => match expr(alu, fields[1]) == expr(alu, fields[2]) {
                true => 1,
                false => 0,
            },
            _ => panic!("unknown instruction {}", fields[0]),
        };

        match fields[1] {
            "w" => alu.w = expr_value,
            "x" => alu.x = expr_value,
            "y" => alu.y = expr_value,
            "z" => alu.z = expr_value,
            _ => panic!("unknown variable {}", fields[1]),
        };
    }
}

fn monad1(instructions: &[String], input: &[i64; 14]) -> i64 {
    let mut alu = ALU {
        input: input.to_vec(),
        index: 0,
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };

    for d in &alu.input {
        assert!(*d != 0, "illegal digit in model number");
    }

    compute(&mut alu, instructions);
    alu.z
}

fn monad2(inp: &[i64; 14]) -> i64 {
    let mut w = inp[0];
    let mut z = w + 8;

    w = inp[1];
    z = z * 26 + w + 13;

    w = inp[2];
    z = z * 26 + w + 2;

    w = inp[3];
    //assert_eq!(z % 26, w);
    z = match z % 26 == w {
        true => z / 26,
        false => z * 26 + w + 7,
    };

    w = inp[4];
    z = z * 26 + w + 11;

    w = inp[5];
    z = z * 26 + w + 4;

    w = inp[6];
    z = z * 26 + w + 13;

    w = inp[7];
    //assert_eq!(z % 26 - 8, w);
    z = match z % 26 - 8 == w {
        true => z / 26,
        false => z * 26 + w + 13,
    };

    w = inp[8];
    //assert_eq!(z % 26 - 9, w);
    z = match z % 26 - 9 == w {
        true => z / 26,
        false => z * 26 + w + 10,
    };

    w = inp[9];
    z = z * 26 + w + 1;

    w = inp[10];
    //assert_eq!(z % 26, w);
    z = match z % 26 == w {
        true => z / 26,
        false => z * 26 + w + 2,
    };

    w = inp[11];
    //assert_eq!(z % 26 - 5, w);
    z = match z % 26 - 5 == w {
        true => z / 26,
        false => z * 26 + w + 14,
    };

    w = inp[12];
    //assert_eq!(z % 26 - 6, w);
    z = match z % 26 - 6 == w {
        true => z / 26,
        false => z * 26 + w + 6,
    };

    w = inp[13];
    //assert_eq!(z % 26 - 12, w);
    z = match z % 26 - 12 == w {
        true => z / 26,
        false => z * 26 + w + 14,
    };
    z
}

fn brute_force(instructions: &[String]) -> Vec<[i64; 14]> {
    let mut solutions = vec![];
    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in &digits {
        let zi = i + 8;
        for j in &digits {
            let zj = zi * 26 + j + 13;
            for k in &digits {
                let zk = zj * 26 + k + 2;
                for l in &digits {
                    if zk % 26 == *l {
                        let zl = zk / 26;
                        for m in &digits {
                            let zm = zl * 26 + m + 11;
                            for n in &digits {
                                let zn = zm * 26 + n + 4;
                                for o in &digits {
                                    let zo = zn * 26 + o + 13;
                                    for p in &digits {
                                        if zo % 26 - 8 == *p {
                                            let zp = zo / 26;
                                            for q in &digits {
                                                if zp % 26 - 9 == *q {
                                                    let zq = zp / 26;
                                                    for r in &digits {
                                                        let zr = zq * 26 + r + 1;
                                                        for s in &digits {
                                                            if zr % 26 == *s {
                                                                let zs = zr / 26;
                                                                for t in &digits {
                                                                    if zs % 26 - 5 == *t {
                                                                        let zt = zs / 26;
                                                                        for u in &digits {
                                                                            if zt % 26 - 6 == *u {
                                                                                let zu = zt / 26;
                                                                                for v in &digits {
                                                                                    if zu % 26 - 12 == *v {
                                                                                        //println!("{},{},{},{},{},{},{},{},{},{},{},{},{},{}", i, j, k, l, m, n, o, p, q, r, s, t, u, v);
                                                                                        assert_eq!(
                                                                                            monad1(
                                                                                                instructions,
                                                                                                &[
                                                                                                    *i, *j, *k, *l, *m,
                                                                                                    *n, *o, *p, *q, *r,
                                                                                                    *s, *t, *u, *v
                                                                                                ]
                                                                                            ),
                                                                                            0
                                                                                        );
                                                                                        assert_eq!(
                                                                                            monad2(&[
                                                                                                *i, *j, *k, *l, *m, *n,
                                                                                                *o, *p, *q, *r, *s, *t,
                                                                                                *u, *v
                                                                                            ]),
                                                                                            0
                                                                                        );
                                                                                        solutions.push([
                                                                                            *i, *j, *k, *l, *m, *n, *o,
                                                                                            *p, *q, *r, *s, *t, *u, *v,
                                                                                        ]);
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
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

    let solutions = brute_force(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {:?}", solutions.last())?;
    writeln!(stdout, "Answer Part 2 = {:?}", solutions.first())?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_actual() {
        let instructions = get_test_data("input-actual");
        let solutions = brute_force(&instructions);
        assert_eq!(Some(&[9, 2, 7, 9, 3, 9, 4, 9, 4, 8, 9, 9, 9, 5]), solutions.last());
        assert_eq!(0, monad1(&instructions, &[9, 2, 7, 9, 3, 9, 4, 9, 4, 8, 9, 9, 9, 5]));
        assert_eq!(0, monad2(&[9, 2, 7, 9, 3, 9, 4, 9, 4, 8, 9, 9, 9, 5]));
    }

    #[test]
    fn part2_actual() {
        let instructions = get_test_data("input-actual");
        let solutions = brute_force(&instructions);
        assert_eq!(Some(&[5, 1, 1, 3, 1, 6, 1, 6, 1, 1, 2, 7, 8, 1]), solutions.first());
        assert_eq!(0, monad1(&instructions, &[5, 1, 1, 3, 1, 6, 1, 6, 1, 1, 2, 7, 8, 1]));
        assert_eq!(0, monad2(&[5, 1, 1, 3, 1, 6, 1, 6, 1, 1, 2, 7, 8, 1]));
    }
}
