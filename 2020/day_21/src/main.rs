use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};
use std::collections::{BTreeMap, HashSet};

fn get_data(data: &[String]) -> Vec<(HashSet<String>, HashSet<String>)> {
    let mut items = vec![];
    for line in data {
        let mut allergens = HashSet::new();
        let mut ingredients = HashSet::new();
        let mut allergy_words = false;
        for word in line.split_whitespace() {
            if word == "(contains" {
                allergy_words = true;
                continue;
            }
            if allergy_words {
                let word = match word.ends_with(')') || word.ends_with(',') {
                    true => word[0..word.len() - 1].to_string(),
                    false => word.to_string(),
                };
                allergens.insert(word);
            } else {
                ingredients.insert(word.to_string());
            }
        }
        items.push((ingredients, allergens));
    }
    items
}

fn solution1(data: &[String]) -> usize {
    let items = get_data(data);
    let mut vmaps = vec![];
    let mut enfoods = HashSet::new();
    for (ingredients, allergens) in &items {
        let mut hmaps = BTreeMap::new();
        for food in allergens {
            hmaps.insert(food.clone(), ingredients.clone());
            enfoods.insert(food.clone());
        }
        vmaps.push(hmaps);
    }

    let mut suspect = HashSet::new();
    for food in &enfoods {
        let f = vmaps
            .iter()
            .filter(|h| h.contains_key(food))
            .map(|h| h[food].clone())
            .collect::<Vec<_>>();

        let mut inter = f[0].clone();
        for thing in f {
            let t = inter.intersection(&thing).collect::<HashSet<_>>();
            inter = t.iter().map(|s| s.to_string()).collect();
        }
        for t in inter.clone() {
            suspect.insert(t.to_string());
        }
    }

    let mut count = 0;
    for (ingredients, _) in items {
        for food in &ingredients {
            if !suspect.contains(food) {
                count += 1;
            }
        }
    }
    count
}

fn solution2(data: &[String]) -> String {
    let items = get_data(data);
    let mut vmaps = vec![];
    let mut enfoods = HashSet::new();
    for (ingredients, allergens) in &items {
        let mut hmaps = BTreeMap::new();
        for food in allergens {
            hmaps.insert(food.clone(), ingredients.clone());
            enfoods.insert(food.clone());
        }
        vmaps.push(hmaps);
    }

    let mut pairings = BTreeMap::<String, Vec<String>>::new();
    for food in &enfoods {
        let f = vmaps
            .iter()
            .filter(|h| h.contains_key(food))
            .map(|h| h[food].clone())
            .collect::<Vec<_>>();

        let mut inter = f[0].clone();
        for thing in f {
            let t = inter.intersection(&thing).collect::<HashSet<_>>();
            inter = t.iter().map(|s| s.to_string()).collect();
        }
        for t in inter.clone() {
            if !pairings.contains_key(food) {
                pairings.insert(food.to_string(), vec![t.to_string()]);
            } else {
                let m = pairings.get_mut(food).unwrap();
                m.push(t.to_string());
            }
        }
    }

    let mut unresolved = true;
    while unresolved {
        let mut tmp_pairings = pairings.clone();
        let singles = pairings
            .values()
            .filter(|v| v.len() == 1)
            .map(|v| v[0].clone())
            .collect::<Vec<_>>();
        for (k, v) in pairings.iter().filter(|(_, v)| v.len() > 1) {
            for s in &singles {
                if let Some(index) = v.iter().position(|r| r == s) {
                    let mut v2 = v.clone();
                    v2.remove(index);
                    tmp_pairings.insert(k.to_string(), v2);
                }
            }
        }
        pairings = tmp_pairings;
        unresolved = pairings.values().filter(|p| p.len() > 1).count() > 0;
    }
    pairings.values().map(|v| v[0].clone()).collect::<Vec<_>>().join(",")
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

    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&puzzle_lines))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&puzzle_lines))?;

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
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(5, solution1(&data));
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(2098, solution1(&data));
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!("mxmxvkd,sqjhc,fvjkl", solution2(&data));
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!("ppdplc,gkcplx,ktlh,msfmt,dqsbql,mvqkdj,ggsz,hbhsx", solution2(&data));
    }
}
