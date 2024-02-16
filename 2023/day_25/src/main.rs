use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    // start a new graph
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let mut edges = vec![];

    // nodes is a map <String, NodeIndex>: "label" => graph.add_node(())
    let mut nodes = HashMap::new();

    // Example graph input:
    //
    // jqt: rhn xhk nvd
    // rsh: frs pzl lsr
    // xhk: hfx
    // cmg: qnr nvd lhk bvb
    // rhn: xhk bvb hfx
    // bvb: xhk hfx
    // pzl: lsr hfx nvd
    // qnr: nvd
    // ntq: jqt hfx bvb xhk
    // nvd: lhk
    // lsr: lhk
    // rzs: qnr cmg lsr rsh
    // frs: qnr lhk lsr
    for line in puzzle_lines {
        let parse = line.split_once(':').unwrap();
        let (node, others) = parse;

        // get the node for the left label
        let left = *nodes.entry(node.to_string()).or_insert_with(|| graph.add_node(()));

        // collect the edges paired with left
        for node in others.split_whitespace() {
            let right = *nodes.entry(node.to_string()).or_insert_with(|| graph.add_node(()));
            edges.push((left, right));
        }
    }

    // create the graph from the edges
    graph.extend_with_edges(edges);

    // perform a minimum cut using Stoer-Wagner: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    // return the product of the lengths of the graphs after the cut
    let partition = min_cut_res?.unwrap().1;
    Ok((graph.node_count() - partition.len()) * partition.len())
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

    writeln!(stdout, "Answer Part 1 = {:?}", part1(&puzzle_lines)?)?;

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
        Ok(read_trimmed_data_lines::<String>(Some(&file))?)
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example")?;
        assert_eq!(part1(&puzzle_lines)?, 54);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual")?;
        assert_eq!(part1(&puzzle_lines)?, 582692);
        Ok(())
    }
}
