use general::{get_args, read_trimmed_data_lines, reset_sigpipe, trim_split_ws};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Write};

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Button,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Module {
    mtype: ModuleType,
    pulse: Pulse,
    dest: Vec<String>,
}

fn get_modules(puzzle_lines: &[String]) -> Result<HashMap<String, Module>, Box<dyn Error>> {
    let mut hmodules = HashMap::new();
    for line in puzzle_lines {
        let parts = trim_split_ws::<String>(line)?;
        let mut names = vec![];
        for label in &parts[2..] {
            names.push(label.replace(',', ""));
        }
        let (name, mtype) = if parts[0] == "broadcaster" {
            ("broadcaster".to_string(), ModuleType::Broadcast)
        } else if parts[0].starts_with('%') {
            (parts[0].replace('%', "").to_string(), ModuleType::FlipFlop)
        } else {
            (parts[0].replace('&', "").to_string(), ModuleType::Conjunction)
        };

        hmodules.insert(
            name,
            Module {
                mtype,
                pulse: Pulse::Low,
                dest: names,
            },
        );
    }

    let button = Module {
        mtype: ModuleType::Button,
        pulse: Pulse::Low,
        dest: vec!["broadcaster".to_string()],
    };
    hmodules.insert("button".to_string(), button);

    Ok(hmodules)
}

fn update_module_pulse(modules: &mut HashMap<String, Module>, name: &str, pulse: Pulse) {
    // update the module pulse if it changed
    if modules[name].pulse != pulse {
        let mut module = modules[name].clone();
        module.pulse = pulse;
        modules.insert(name.to_string(), module);
    }
}

fn solution(puzzle_lines: &[String], p2: bool) -> Result<usize, Box<dyn Error>> {
    let mut modules = get_modules(puzzle_lines)?;

    // initialize an empty map of inputs for each conjunction module
    let mut conjunctions = modules
        .iter()
        .filter(|(_, v)| matches!(v.mtype, ModuleType::Conjunction))
        .map(|(k, _)| (k.to_string(), HashMap::<String, Pulse>::new()))
        .collect::<HashMap<String, HashMap<String, Pulse>>>();

    // populate the map with the inputs to each conjunction, initialized with a low pulse
    for (k, v) in &modules {
        for d in &v.dest {
            if let Some(h) = conjunctions.get_mut(d) {
                h.insert(k.to_string(), Pulse::Low);
            }
        }
    }

    let mut cycle = HashMap::new();
    let mut counts = HashMap::new();

    // Part 2 asks when the inputs to conjunction "rx" will all be low at the same time
    // so we need to find the cycle length of each and multiply
    let rx = modules.iter().find(|(_, v)| v.dest.contains(&"rx".to_string()));
    // &ll -> rx
    // kv, vm, kl, vb -> ll
    //
    // println!("rx_inputs = {rx_inputs:?}");
    // ["kv", "vm", "kl", "vb"]
    let rx_inputs = match rx {
        Some(m) => modules
            .iter()
            .filter(|(_, v)| v.dest.contains(m.0))
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>(),
        None => vec![],
    };

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut lcm: usize = 1;
    for button_push in 0..10000 {
        let mut workq = VecDeque::new();

        // initialize the workq with a button press
        workq.push_back(
            //     sender,           pulse,            receiver
            ("button".to_string(), Pulse::Low, "broadcaster".to_string()),
        );

        while let Some(msg) = workq.pop_front() {
            let (sender, pulse, receiver) = msg;

            // Part1: Consult your module configuration; determine the number of low pulses and high pulses
            // that would be sent after pushing the button 1000 times, waiting for all pulses to be fully
            // handled after each push of the button. What do you get if you multiply the total number of
            // low pulses sent by the total number of high pulses sent?
            if button_push == 1000 && !p2 {
                return Ok(low_pulse_count * high_pulse_count);
            }

            if pulse == Pulse::Low {
                low_pulse_count += 1;

                // Reset all modules to their default states.
                // Waiting for all pulses to be fully handled after each button press,
                // what is the fewest number of button presses required to deliver a
                // single low pulse to the module named rx?
                if p2 {
                    if rx_inputs.contains(&receiver) {
                        *counts.entry(receiver.to_string()).or_insert(0) += 1;
                        // On the 2nd instance of each receiver, accumulate the product of its cycle length
                        if counts[&receiver] == 2 {
                            //lcm *= (button_push - cycle[&receiver]) as usize;
                            lcm = num_integer::lcm(lcm, (button_push - cycle[&receiver]) as usize);
                        } else {
                            cycle.entry(receiver.to_string()).or_insert(button_push);
                        }
                    }
                    // stop when the 2nd instance of all inputs have been seen & accumulated
                    if counts.len() == rx_inputs.len() && counts.values().all(|c| *c == 2) {
                        return Ok(lcm);
                    }
                }
            } else {
                high_pulse_count += 1;
            }

            //
            // this section just sends pulses according to the module rules
            //
            if let Some(module) = modules.get(&receiver) {
                let module = module.clone();
                let mut send_pulse = module.pulse;

                match module.mtype {
                    // Flip-flop modules (prefix %) are either on or off; they are initially off.
                    // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                    // However, if a flip-flop module receives a low pulse, it flips between on and off.
                    // If it was off, it turns on and sends a high pulse.
                    // If it was on, it turns off and sends a low pulse.
                    ModuleType::FlipFlop => match pulse {
                        Pulse::High => continue,
                        Pulse::Low => {
                            send_pulse = match module.pulse {
                                Pulse::Low => Pulse::High,
                                Pulse::High => Pulse::Low,
                            };
                        }
                    },
                    // Conjunction modules (prefix &) remember the type of the most recent pulse
                    // received from each of their connected input modules; they initially default
                    // to remembering a low pulse for each input.
                    // When a pulse is received, the conjunction module first updates its memory
                    // for that input. Then, if it remembers high pulses for all inputs,
                    // it sends a low pulse; otherwise, it sends a high pulse.
                    ModuleType::Conjunction => {
                        let conj = conjunctions.get_mut(&receiver).unwrap();
                        conj.insert(sender.to_string(), pulse);
                        send_pulse = match conj.values().all(|p| *p == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                    }
                    // There is a single broadcast module (named broadcaster).
                    // When it receives a pulse, it sends the same pulse to all of its destination modules.
                    //
                    // Here at Desert Machine Headquarters, there is a module with a single button
                    // on it called, aptly, the button module. When you push the button,
                    // a single low pulse is sent directly to the broadcaster module.
                    ModuleType::Broadcast | ModuleType::Button => (),
                }

                // update the receiver module
                update_module_pulse(&mut modules, &receiver, send_pulse);

                // send a pulse to each destination module
                for m in &module.dest {
                    workq.push_back((receiver.to_string(), send_pulse, m.to_string()));
                }
            }
        }
    }
    // NOTREACHED
    Ok(0)
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, false)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solution(puzzle_lines, true)
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
        assert_eq!(part1(&puzzle_lines)?, 32000000);
        Ok(())
    }

    #[test]
    fn part1_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2")?;
        assert_eq!(part1(&puzzle_lines)?, 11687500);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 743090292);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part2(&puzzle_lines)?, 241528184647003);
        Ok(())
    }
}
