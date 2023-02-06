use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

fn get_graph(data: &[String]) -> BTreeMap<String, HashSet<String>> {
    // parsing rules for data[String]
    // Example:
    //   "start-A",
    //   "start-b",
    //   "A-c",
    //   "A-b",
    //   "b-d",
    //   "A-end",
    //   "b-end"
    // Returns:
    //   graph = {
    //      "start": {"b", "A"},
    //      "b": {"d", "A", "end"},
    //      "A": {"c", "b", "end"}}
    //      "d": {"b"},
    //      "c": {"A"},
    //      "end": {"A", "b"},
    //   }

    let mut graph = BTreeMap::new();
    for s in data {
        let nodes = s.trim().split('-').map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(nodes.len(), 2, "expected 2 nodes: {nodes:?}");
        let (a, b) = (nodes[0].clone(), nodes[1].clone());
        let h = graph.entry(a.clone()).or_insert_with(HashSet::new);
        h.insert(b.clone());
        if a != "start" && b != "end" {
            let h = graph.entry(b).or_insert_with(HashSet::new);
            h.insert(a);
        }
    }
    graph
}

fn is_small(s: &str) -> bool {
    s.to_lowercase() == s
}

fn visit(
    graph: &BTreeMap<String, HashSet<String>>,
    node: &str,
    special: &str,
    max_count: usize,
    visited: &mut BTreeMap<String, usize>,
    paths: &mut Vec<String>,
    solutions: &mut HashSet<Vec<String>>,
) {
    if node == "end" {
        //println!("paths = {:?}", paths);
        solutions.insert(paths.to_vec());
        return;
    }

    if is_small(node) {
        let count = visited.entry(node.to_string()).or_insert(0);
        *count += 1;
    }

    match graph.get(node) {
        Some(items) => {
            for item in items {
                if !visited.contains_key(item) || (item == special && visited.get(special) < Some(&max_count)) {
                    paths.push(item.to_string());
                    visit(graph, item, special, max_count, visited, paths, solutions);
                    paths.pop();
                    if let Some(count) = visited.get_mut(item) {
                        *count -= 1;
                        if *count == 0 {
                            visited.remove(item);
                        }
                    }
                }
            }
        }
        None => panic!("expected node = {node} to be in graph"),
    }
}

fn solution(graph: &BTreeMap<String, HashSet<String>>, count: usize) -> usize {
    let mut paths = vec![];
    let mut visited = BTreeMap::new();
    let mut solutions = HashSet::new();
    if count == 1 {
        visit(graph, "start", "end", count, &mut visited, &mut paths, &mut solutions);
    } else {
        for k in graph.keys() {
            if is_small(k) && k != "start" && k != "end" {
                visit(graph, "start", k, count, &mut visited, &mut paths, &mut solutions);
            }
        }
    }
    solutions.len()
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

    let graph = get_graph(&puzzle_lines);
    writeln!(stdout, "Answer Part 1 = {}", solution(&graph, 1))?;
    writeln!(stdout, "Answer Part 2 = {}", solution(&graph, 2))?;

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
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 1), 10);
    }

    #[test]
    fn part1_example2() {
        let data = get_data("input-example2");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 1), 19);
    }

    #[test]
    fn part1_example3() {
        let data = get_data("input-example3");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 1), 226);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 1), 4186);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 2), 36);
    }

    #[test]
    fn part2_example2() {
        let data = get_data("input-example2");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 2), 103);
    }

    #[test]
    fn part2_example3() {
        let data = get_data("input-example3");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 2), 3509);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let graph = get_graph(&data);
        assert_eq!(solution(&graph, 2), 92111);
    }
}
